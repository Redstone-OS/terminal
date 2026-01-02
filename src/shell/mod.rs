//! # Shell Module
//!
//! Shell interativo do terminal RedstoneOS.
//!
//! ## Comandos Suportados
//!
//! | Comando | Descrição | Status |
//! |---------|-----------|--------|
//! | `ls` | Lista arquivos | 🟢 |
//! | `cd` | Muda diretório | 🟢 |
//! | `pwd` | Mostra diretório atual | 🟢 |
//! | `cat` | Mostra conteúdo de arquivo | 🟢 |
//! | `tree` | Mostra árvore de diretórios | 🟢 |
//! | `mkdir` | Cria diretório | ⚪ |
//! | `rmdir` | Remove diretório | ⚪ |
//! | `rm` | Remove arquivo | ⚪ |
//! | `cp` | Copia arquivo | ⚪ |
//! | `mv` | Move/renomeia arquivo | ⚪ |
//! | `clear` | Limpa tela | 🟢 |
//! | `exit` | Sai do terminal | 🟢 |
//! | `help` | Mostra ajuda | 🟢 |
//! | `uptime` | Tempo desde boot | 🟡 |
//! | `ps` | Lista processos | ⚪ |
//! | `kill` | Mata processo | ⚪ |
//! | `top` | Monitor de processos | ⚪ |
//! | `jobs` | Lista jobs | ⚪ |
//! | `sysinfo` | Info do sistema | ⚪ |
//! | `meminfo` | Info de memória | ⚪ |

pub mod commands;
mod context;

pub use commands::execute_command;
pub use context::ShellContext;
