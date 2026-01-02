//! # System Commands
//!
//! Comandos de sistema.

use crate::state::terminal::TerminalState;

// =============================================================================
// uptime - Tempo desde boot
// =============================================================================

pub fn cmd_uptime(output: &mut TerminalState) {
    // Por enquanto, usar syscall de clock
    match redpowder::time::clock() {
        Ok(ticks) => {
            // Assumindo ticks em ms
            let total_secs = ticks / 1000;
            let hours = total_secs / 3600;
            let mins = (total_secs % 3600) / 60;
            let secs = total_secs % 60;

            output.write_str("Uptime: ");
            write_number(output, hours);
            output.write_str("h ");
            write_number(output, mins);
            output.write_str("m ");
            write_number(output, secs);
            output.write_line("s");
        }
        Err(_) => {
            output.write_line("uptime: Nao foi possivel obter tempo");
        }
    }
}

// =============================================================================
// ps - Lista processos
// =============================================================================

pub fn cmd_ps(output: &mut TerminalState, args: &[&str]) {
    let json_output = args.contains(&"--json");

    if json_output {
        output.write_line("ps --json: Nao implementado");
        output.write_line("(Contrato futuro para integracao com ferramentas)");
        return;
    }

    output.write_line("ps: Nao implementado");
    output.write_line("(Requer syscall de listagem de processos)");
    output.write_line("");
    output.write_line("Formato futuro:");
    output.write_line("  PID  PPID  STATE  NAME");
    output.write_line("    1     0  R      supervisor");
    output.write_line("    2     1  R      firefly");
    output.write_line("    3     2  R      terminal");
}

// =============================================================================
// kill - Mata processo
// =============================================================================

pub fn cmd_kill(output: &mut TerminalState, args: &[&str]) {
    if args.is_empty() {
        output.write_line("kill: falta PID");
        output.write_line("Uso: kill <pid>");
        return;
    }

    output.write_line("kill: Nao implementado");
    output.write_line("(Requer syscall de sinais)");
}

// =============================================================================
// top - Monitor de processos
// =============================================================================

pub fn cmd_top(output: &mut TerminalState) {
    output.write_line("top: Nao implementado");
    output.write_line("(Requer syscall de estatisticas de processos)");
    output.write_line("");
    output.write_line("Formato futuro:");
    output.write_line("  CPU: 12%  MEM: 45MB/256MB");
    output.write_line("  PID  CPU%  MEM%  NAME");
    output.write_line("    1   2%    5%  supervisor");
    output.write_line("    2   8%   15%  firefly");
}

// =============================================================================
// jobs - Lista jobs
// =============================================================================

pub fn cmd_jobs(output: &mut TerminalState) {
    output.write_line("jobs: Nao implementado");
    output.write_line("(Requer suporte a job control)");
}

// =============================================================================
// sysinfo - Info do sistema
// =============================================================================

pub fn cmd_sysinfo(output: &mut TerminalState) {
    output.write_line("");
    output.write_line("=== RedstoneOS System Info ===");
    output.write_line("");
    output.write_line("  OS:        RedstoneOS v0.1.3");
    output.write_line("  Kernel:    Forge Microkernel");
    output.write_line("  Arch:      x86_64");
    output.write_line("  Shell:     Firefly Terminal v0.2.0");
    output.write_line("");

    // Uptime
    output.write_str("  Uptime:    ");
    match redpowder::time::clock() {
        Ok(ticks) => {
            let secs = ticks / 1000;
            write_number(output, secs);
            output.write_line(" segundos");
        }
        Err(_) => output.write_line("(desconhecido)"),
    }

    output.write_line("");
    output.write_line("(sysinfo completo requer syscall SYS_SYSINFO)");
}

// =============================================================================
// meminfo - Info de memória
// =============================================================================

pub fn cmd_meminfo(output: &mut TerminalState) {
    output.write_line("");
    output.write_line("=== Memory Info ===");
    output.write_line("");
    output.write_line("meminfo: Nao implementado");
    output.write_line("(Requer syscall de estatisticas de memoria)");
    output.write_line("");
    output.write_line("Formato futuro:");
    output.write_line("  Total:     256 MB");
    output.write_line("  Used:       45 MB");
    output.write_line("  Free:      211 MB");
    output.write_line("  Cached:     12 MB");
    output.write_line("  Buffers:     5 MB");
}

// =============================================================================
// HELPER
// =============================================================================

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
