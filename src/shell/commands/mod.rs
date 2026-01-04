//! # Shell Commands
//!
//! Dispatcher e implementação de comandos do shell.

mod builtin;
mod fs;
mod system;

use super::ShellContext;
use crate::state::terminal::TerminalState;
use alloc::string::String;
use alloc::vec::Vec;

// TODO: Revisar no futuro
#[allow(unused)]
/// Resultado de execução de comando
pub enum CommandResult {
    /// Comando executado com sucesso
    Ok,
    /// Comando pede para sair do terminal
    Exit,
    /// Comando pede para limpar a tela
    Clear,
    /// Erro com mensagem
    Error(String),
}

/// Executa um comando
pub fn execute_command(
    cmd_line: &str,
    ctx: &mut ShellContext,
    output: &mut TerminalState,
) -> CommandResult {
    // Fazer trim e verificar se está vazio
    let cmd_line = cmd_line.trim();
    if cmd_line.is_empty() {
        return CommandResult::Ok;
    }

    // Parsear argumentos (split por espaços, respeitando aspas simples)
    let args = parse_args(cmd_line);
    if args.is_empty() {
        return CommandResult::Ok;
    }

    let cmd = args[0].as_str();
    let args: Vec<&str> = args.iter().skip(1).map(|s| s.as_str()).collect();

    // Dispatcher de comandos
    match cmd {
        // === BUILTIN ===
        "help" => builtin::cmd_help(output, &args),
        "clear" => return CommandResult::Clear,
        "exit" | "quit" => return CommandResult::Exit,
        "echo" => builtin::cmd_echo(output, &args),
        "ver" | "version" => builtin::cmd_version(output),

        // === FILESYSTEM ===
        "ls" | "dir" => fs::cmd_ls(output, ctx, &args),
        "cd" => fs::cmd_cd(output, ctx, &args),
        "pwd" => fs::cmd_pwd(output, ctx),
        "cat" | "type" => fs::cmd_cat(output, ctx, &args),
        "tree" => fs::cmd_tree(output, ctx, &args),
        "mkdir" => fs::cmd_mkdir(output, ctx, &args),
        "rmdir" => fs::cmd_rmdir(output, ctx, &args),
        "rm" | "del" => fs::cmd_rm(output, ctx, &args),
        "cp" | "copy" => fs::cmd_cp(output, ctx, &args),
        "mv" | "move" | "rename" => fs::cmd_mv(output, ctx, &args),
        "stat" => fs::cmd_stat(output, ctx, &args),

        // === SYSTEM ===
        "uptime" => system::cmd_uptime(output),
        "ps" => system::cmd_ps(output, &args),
        "kill" => system::cmd_kill(output, &args),
        "top" => system::cmd_top(output),
        "jobs" => system::cmd_jobs(output),
        "sysinfo" => system::cmd_sysinfo(output),
        "meminfo" => system::cmd_meminfo(output),

        // === DESCONHECIDO ===
        _ => {
            output.write_str("Comando nao encontrado: ");
            output.write_line(cmd);
            output.write_line("Digite 'help' para ver comandos disponiveis.");
        }
    }

    CommandResult::Ok
}

/// Parseia argumentos de uma linha de comando
fn parse_args(cmd_line: &str) -> Vec<String> {
    let mut args = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;
    let mut quote_char = ' ';

    for c in cmd_line.chars() {
        match c {
            '"' | '\'' if !in_quotes => {
                in_quotes = true;
                quote_char = c;
            }
            c if in_quotes && c == quote_char => {
                in_quotes = false;
            }
            ' ' | '\t' if !in_quotes => {
                if !current.is_empty() {
                    args.push(current.clone());
                    current.clear();
                }
            }
            _ => {
                current.push(c);
            }
        }
    }

    if !current.is_empty() {
        args.push(current);
    }

    args
}
