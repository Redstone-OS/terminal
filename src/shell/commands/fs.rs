//! # Filesystem Commands
//!
//! Comandos de sistema de arquivos.

use crate::shell::ShellContext;
use crate::state::terminal::TerminalState;
use alloc::string::String;
use alloc::vec::Vec;
use redpowder::fs::{chdir, exists, getcwd, is_dir, stat, Dir, File};

// =============================================================================
// ls - Lista arquivos
// =============================================================================

pub fn cmd_ls(output: &mut TerminalState, ctx: &mut ShellContext, args: &[&str]) {
    let mut show_details = false;
    let mut show_hidden = false;
    let mut json_output = false;
    let mut path = ctx.cwd.as_str();

    // Parsear argumentos
    for arg in args {
        match *arg {
            "-l" => show_details = true,
            "-a" => show_hidden = true,
            "--json" => json_output = true,
            "-la" | "-al" => {
                show_details = true;
                show_hidden = true;
            }
            _ if !arg.starts_with('-') => path = arg,
            _ => {
                output.write_str("ls: opcao desconhecida: ");
                output.write_line(arg);
                return;
            }
        }
    }

    // JSON output - futuro
    if json_output {
        output.write_line("ls --json: Nao implementado ainda");
        return;
    }

    // Resolver path
    let full_path = resolve_path(&ctx.cwd, path);

    // Abrir diretório
    match Dir::open(&full_path) {
        Ok(dir) => {
            let mut entries: Vec<(String, bool, u64)> = Vec::new();

            for entry in dir.entries() {
                let name = entry.name();

                // Pular . e .. e arquivos ocultos (se não -a)
                if !show_hidden && name.starts_with('.') {
                    continue;
                }

                entries.push((
                    String::from(name),
                    entry.is_dir(),
                    0, // tamanho - precisaria de stat individual
                ));
            }

            // Ordenar: diretórios primeiro, depois alfabético
            entries.sort_by(|a, b| match (a.1, b.1) {
                (true, false) => core::cmp::Ordering::Less,
                (false, true) => core::cmp::Ordering::Greater,
                _ => a.0.cmp(&b.0),
            });

            if entries.is_empty() {
                output.write_line("(diretorio vazio)");
                return;
            }

            if show_details {
                // Formato detalhado
                output.write_line("TIPO  TAMANHO  NOME");
                output.write_line("----  -------  ----");
                for (name, is_dir, size) in entries {
                    let type_str = if is_dir { "DIR " } else { "FILE" };
                    output.write_str(type_str);
                    output.write_str("  ");
                    // Tamanho (placeholder)
                    output.write_str("       -");
                    output.write_str("  ");
                    if is_dir {
                        output.write_str("[");
                        output.write_str(&name);
                        output.write_line("]");
                    } else {
                        output.write_line(&name);
                    }
                }
            } else {
                // Formato simples
                let mut line_len = 0;
                for (name, is_dir, _) in entries {
                    let display_name = if is_dir {
                        let mut s = String::from("[");
                        s.push_str(&name);
                        s.push(']');
                        s
                    } else {
                        name.clone()
                    };

                    // Quebrar linha se necessário
                    if line_len + display_name.len() + 2 > 70 && line_len > 0 {
                        output.write_line("");
                        line_len = 0;
                    }

                    if line_len > 0 {
                        output.write_str("  ");
                        line_len += 2;
                    }

                    output.write_str(&display_name);
                    line_len += display_name.len();
                }
                output.write_line("");
            }
        }
        Err(e) => {
            output.write_str("ls: nao foi possivel abrir ");
            output.write_str(&full_path);
            output.write_str(": ");
            output.write_line(error_to_str(e));
        }
    }
}

// =============================================================================
// cd - Muda diretório
// =============================================================================

pub fn cmd_cd(output: &mut TerminalState, ctx: &mut ShellContext, args: &[&str]) {
    let path = args.get(0).unwrap_or(&"/");

    // Resolver path
    let full_path = resolve_path(&ctx.cwd, path);

    // Verificar se existe e é diretório
    if !exists(&full_path) {
        output.write_str("cd: ");
        output.write_str(&full_path);
        output.write_line(": Nao existe");
        return;
    }

    if !is_dir(&full_path) {
        output.write_str("cd: ");
        output.write_str(&full_path);
        output.write_line(": Nao e um diretorio");
        return;
    }

    // Mudar diretório via syscall
    match chdir(&full_path) {
        Ok(_) => {
            ctx.set_cwd(&full_path);
        }
        Err(e) => {
            output.write_str("cd: ");
            output.write_str(&full_path);
            output.write_str(": ");
            output.write_line(error_to_str(e));
        }
    }
}

// =============================================================================
// pwd - Mostra diretório atual
// =============================================================================

pub fn cmd_pwd(output: &mut TerminalState, ctx: &ShellContext) {
    // Tentar via syscall
    let mut buf = [0u8; 256];
    match getcwd(&mut buf) {
        Ok(cwd) => output.write_line(cwd),
        Err(_) => output.write_line(&ctx.cwd),
    }
}

// =============================================================================
// cat - Mostra conteúdo de arquivo
// =============================================================================

pub fn cmd_cat(output: &mut TerminalState, ctx: &mut ShellContext, args: &[&str]) {
    if args.is_empty() {
        output.write_line("cat: falta operando arquivo");
        output.write_line("Uso: cat <arquivo>");
        return;
    }

    for arg in args {
        let full_path = resolve_path(&ctx.cwd, arg);

        match File::open(&full_path) {
            Ok(file) => {
                let mut buf = [0u8; 512];
                loop {
                    match file.read(&mut buf) {
                        Ok(0) => break, // EOF
                        Ok(n) => {
                            // Converter para string e imprimir
                            if let Ok(text) = core::str::from_utf8(&buf[..n]) {
                                for c in text.chars() {
                                    if c == '\n' {
                                        output.write_line("");
                                    } else if c >= ' ' || c == '\t' {
                                        output.write_char(c);
                                    }
                                }
                            } else {
                                output.write_line("(conteudo binario nao exibido)");
                                break;
                            }
                        }
                        Err(e) => {
                            output.write_str("cat: erro ao ler: ");
                            output.write_line(error_to_str(e));
                            break;
                        }
                    }
                }
            }
            Err(e) => {
                output.write_str("cat: ");
                output.write_str(&full_path);
                output.write_str(": ");
                output.write_line(error_to_str(e));
            }
        }
    }
}

// =============================================================================
// tree - Árvore de diretórios
// =============================================================================

pub fn cmd_tree(output: &mut TerminalState, ctx: &mut ShellContext, args: &[&str]) {
    let mut path = ctx.cwd.as_str();
    let mut max_depth = 3usize;

    // Parsear argumentos
    let mut i = 0;
    while i < args.len() {
        match args[i] {
            "-d" => {
                if i + 1 < args.len() {
                    if let Ok(d) = args[i + 1].parse::<usize>() {
                        max_depth = d;
                    }
                    i += 1;
                }
            }
            arg if !arg.starts_with('-') => path = arg,
            _ => {}
        }
        i += 1;
    }

    let full_path = resolve_path(&ctx.cwd, path);
    output.write_line(&full_path);
    tree_recursive(output, &full_path, "", 0, max_depth);
}

fn tree_recursive(
    output: &mut TerminalState,
    path: &str,
    prefix: &str,
    depth: usize,
    max_depth: usize,
) {
    if depth >= max_depth {
        return;
    }

    let dir = match Dir::open(path) {
        Ok(d) => d,
        Err(_) => return,
    };

    let mut entries: Vec<(String, bool)> = Vec::new();
    for entry in dir.entries() {
        let name = entry.name();
        if name == "." || name == ".." {
            continue;
        }
        entries.push((String::from(name), entry.is_dir()));
    }

    entries.sort_by(|a, b| a.0.cmp(&b.0));
    let count = entries.len();

    for (i, (name, is_dir)) in entries.iter().enumerate() {
        let is_last = i == count - 1;
        let connector = if is_last { "└── " } else { "├── " };

        output.write_str(prefix);
        output.write_str(connector);
        if *is_dir {
            output.write_str("[");
            output.write_str(name);
            output.write_line("]");

            // Recursar
            let child_prefix = if is_last {
                let mut p = String::from(prefix);
                p.push_str("    ");
                p
            } else {
                let mut p = String::from(prefix);
                p.push_str("│   ");
                p
            };

            let child_path = join_path(path, name);
            tree_recursive(output, &child_path, &child_prefix, depth + 1, max_depth);
        } else {
            output.write_line(name);
        }
    }
}

// =============================================================================
// stat - Info de arquivo
// =============================================================================

pub fn cmd_stat(output: &mut TerminalState, ctx: &mut ShellContext, args: &[&str]) {
    if args.is_empty() {
        output.write_line("stat: falta operando");
        return;
    }

    for arg in args {
        let full_path = resolve_path(&ctx.cwd, arg);

        match stat(&full_path) {
            Ok(info) => {
                output.write_str("  Arquivo: ");
                output.write_line(&full_path);

                output.write_str("     Tipo: ");
                let type_str = match info.file_type {
                    1 => "arquivo regular",
                    2 => "diretorio",
                    3 => "link simbolico",
                    _ => "desconhecido",
                };
                output.write_line(type_str);

                output.write_str("  Tamanho: ");
                write_number(output, info.size);
                output.write_line(" bytes");

                output.write_str("     Mode: ");
                write_number(output, info.mode as u64);
                output.write_line("");
            }
            Err(e) => {
                output.write_str("stat: ");
                output.write_str(&full_path);
                output.write_str(": ");
                output.write_line(error_to_str(e));
            }
        }
    }
}

// =============================================================================
// STUBS - Comandos não implementados
// =============================================================================

pub fn cmd_mkdir(output: &mut TerminalState, _ctx: &mut ShellContext, _args: &[&str]) {
    output.write_line("mkdir: Nao implementado");
    output.write_line("(O filesystem ainda e somente leitura)");
}

pub fn cmd_rmdir(output: &mut TerminalState, _ctx: &mut ShellContext, _args: &[&str]) {
    output.write_line("rmdir: Nao implementado");
    output.write_line("(O filesystem ainda e somente leitura)");
}

pub fn cmd_rm(output: &mut TerminalState, _ctx: &mut ShellContext, _args: &[&str]) {
    output.write_line("rm: Nao implementado");
    output.write_line("(O filesystem ainda e somente leitura)");
}

pub fn cmd_cp(output: &mut TerminalState, _ctx: &mut ShellContext, _args: &[&str]) {
    output.write_line("cp: Nao implementado");
    output.write_line("(O filesystem ainda e somente leitura)");
}

pub fn cmd_mv(output: &mut TerminalState, _ctx: &mut ShellContext, _args: &[&str]) {
    output.write_line("mv: Nao implementado");
    output.write_line("(O filesystem ainda e somente leitura)");
}

// =============================================================================
// HELPERS
// =============================================================================

/// Resolve um path relativo ao CWD
fn resolve_path(cwd: &str, path: &str) -> String {
    if path.starts_with('/') {
        // Path absoluto
        normalize_path(path)
    } else {
        // Path relativo
        let mut full = String::from(cwd.trim_end_matches('/'));
        full.push('/');
        full.push_str(path);
        normalize_path(&full)
    }
}

/// Normaliza um path (remove //, ., ..)
fn normalize_path(path: &str) -> String {
    let mut components: Vec<&str> = Vec::new();

    for comp in path.split('/') {
        match comp {
            "" | "." => continue,
            ".." => {
                components.pop();
            }
            _ => components.push(comp),
        }
    }

    let mut result = String::from("/");
    for (i, comp) in components.iter().enumerate() {
        if i > 0 {
            result.push('/');
        }
        result.push_str(comp);
    }

    if result.is_empty() {
        String::from("/")
    } else {
        result
    }
}

/// Junta dois paths
fn join_path(base: &str, child: &str) -> String {
    let mut path = String::from(base.trim_end_matches('/'));
    path.push('/');
    path.push_str(child);
    path
}

/// Converte erro para string
fn error_to_str(e: redpowder::SysError) -> &'static str {
    match e {
        redpowder::SysError::NotFound => "Nao encontrado",
        redpowder::SysError::PermissionDenied => "Permissao negada",
        redpowder::SysError::IsDirectory => "E um diretorio",
        redpowder::SysError::NotDirectory => "Nao e um diretorio",
        redpowder::SysError::NotImplemented => "Nao implementado",
        redpowder::SysError::IoError => "Erro de E/S",
        _ => "Erro desconhecido",
    }
}

/// Escreve número no output (sem alloc)
fn write_number(output: &mut TerminalState, n: u64) {
    if n == 0 {
        output.write_str("0");
        return;
    }

    let mut buf = [0u8; 20];
    let mut i = 20;
    let mut val = n;

    while val > 0 && i > 0 {
        i -= 1;
        buf[i] = b'0' + (val % 10) as u8;
        val /= 10;
    }

    if let Ok(s) = core::str::from_utf8(&buf[i..]) {
        output.write_str(s);
    }
}
