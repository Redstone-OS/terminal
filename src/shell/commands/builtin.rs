//! # Builtin Commands
//!
//! Comandos internos do shell.

use crate::state::terminal::TerminalState;

/// help - Mostra ajuda
pub fn cmd_help(output: &mut TerminalState, args: &[&str]) {
    // Se tiver argumento, mostra ajuda específica
    if !args.is_empty() {
        show_command_help(output, args[0]);
        return;
    }

    output.write_line("");
    output.write_line("=== RedstoneOS Terminal v0.2.0 ===");
    output.write_line("");
    output.write_line("COMANDOS DE ARQUIVOS:");
    output.write_line("  ls [path]        Lista arquivos e diretorios");
    output.write_line("  cd <path>        Muda diretorio atual");
    output.write_line("  pwd              Mostra diretorio atual");
    output.write_line("  cat <file>       Mostra conteudo de arquivo");
    output.write_line("  tree [path]      Mostra arvore de diretorios");
    output.write_line("  stat <path>      Mostra informacoes de arquivo");
    output.write_line("  mkdir <path>     Cria diretorio");
    output.write_line("  rmdir <path>     Remove diretorio vazio");
    output.write_line("  rm <file>        Remove arquivo");
    output.write_line("  cp <src> <dst>   Copia arquivo");
    output.write_line("  mv <src> <dst>   Move/renomeia arquivo");
    output.write_line("");
    output.write_line("COMANDOS DE SISTEMA:");
    output.write_line("  uptime           Tempo desde boot");
    output.write_line("  ps [--json]      Lista processos");
    output.write_line("  kill <pid>       Mata processo");
    output.write_line("  top              Monitor de processos");
    output.write_line("  jobs             Lista jobs");
    output.write_line("  sysinfo          Informacoes do sistema");
    output.write_line("  meminfo          Informacoes de memoria");
    output.write_line("");
    output.write_line("OUTROS:");
    output.write_line("  help [cmd]       Mostra esta ajuda");
    output.write_line("  clear            Limpa a tela");
    output.write_line("  exit             Sai do terminal");
    output.write_line("  echo <text>      Imprime texto");
    output.write_line("  ver              Versao do sistema");
    output.write_line("");
    output.write_line("FLAGS FUTURAS:");
    output.write_line("  ls --json        Saida em formato JSON");
    output.write_line("  ps --json        Saida em formato JSON");
    output.write_line("");
}

/// Mostra ajuda de um comando específico
fn show_command_help(output: &mut TerminalState, cmd: &str) {
    match cmd {
        "ls" => {
            output.write_line("ls - Lista arquivos e diretorios");
            output.write_line("");
            output.write_line("USO: ls [opcoes] [caminho]");
            output.write_line("");
            output.write_line("OPCOES:");
            output.write_line("  -l         Lista detalhada");
            output.write_line("  -a         Mostra arquivos ocultos");
            output.write_line("  --json     Saida em JSON (futuro)");
            output.write_line("");
            output.write_line("EXEMPLOS:");
            output.write_line("  ls");
            output.write_line("  ls /apps");
            output.write_line("  ls -l /system");
        }
        "cd" => {
            output.write_line("cd - Muda diretorio atual");
            output.write_line("");
            output.write_line("USO: cd <caminho>");
            output.write_line("");
            output.write_line("EXEMPLOS:");
            output.write_line("  cd /apps");
            output.write_line("  cd ..");
            output.write_line("  cd /");
        }
        "cat" => {
            output.write_line("cat - Mostra conteudo de arquivo");
            output.write_line("");
            output.write_line("USO: cat <arquivo>");
            output.write_line("");
            output.write_line("EXEMPLOS:");
            output.write_line("  cat /apps/config.txt");
        }
        "tree" => {
            output.write_line("tree - Mostra arvore de diretorios");
            output.write_line("");
            output.write_line("USO: tree [caminho] [opcoes]");
            output.write_line("");
            output.write_line("OPCOES:");
            output.write_line("  -d <n>     Profundidade maxima");
            output.write_line("");
            output.write_line("EXEMPLOS:");
            output.write_line("  tree");
            output.write_line("  tree /system");
            output.write_line("  tree / -d 2");
        }
        _ => {
            output.write_str("Ajuda nao disponivel para: ");
            output.write_line(cmd);
        }
    }
}

/// echo - Imprime texto
pub fn cmd_echo(output: &mut TerminalState, args: &[&str]) {
    for (i, arg) in args.iter().enumerate() {
        if i > 0 {
            output.write_str(" ");
        }
        output.write_str(arg);
    }
    output.write_line("");
}

/// version - Mostra versão
pub fn cmd_version(output: &mut TerminalState) {
    output.write_line("");
    output.write_line("RedstoneOS v0.1.3 (Forge Kernel)");
    output.write_line("Terminal v0.2.0 (Firefly)");
    output.write_line("SDK v0.2.0 (Redpowder)");
    output.write_line("");
    output.write_line("Copyright (c) 2026 RedstoneOS Team");
    output.write_line("");
}
