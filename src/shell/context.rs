//! # Shell Context
//!
//! Contexto compartilhado do shell.

use alloc::string::String;

// TODO: Revisar no futuro
#[allow(unused)]
/// Contexto do shell
pub struct ShellContext {
    /// Diretório de trabalho atual
    pub cwd: String,
    /// Último código de saída
    pub last_exit_code: i32,
    /// Nome do usuário
    pub username: String,
    /// Hostname
    pub hostname: String,
}

impl ShellContext {
    /// Cria novo contexto
    pub fn new() -> Self {
        Self {
            cwd: String::from("/"),
            last_exit_code: 0,
            username: String::from("redstone"),
            hostname: String::from("localhost"),
        }
    }

    /// Retorna o prompt formatado
    pub fn prompt(&self) -> String {
        use alloc::format;
        format!("{}@{}:{}$ ", self.username, self.hostname, self.cwd)
    }

    /// Atualiza CWD
    pub fn set_cwd(&mut self, path: &str) {
        self.cwd.clear();
        self.cwd.push_str(path);
    }
}

impl Default for ShellContext {
    fn default() -> Self {
        Self::new()
    }
}
