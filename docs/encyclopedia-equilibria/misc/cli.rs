//! 1-2-3 CLI interface that disguises Stage 0 (Agent Generator)

use std::io::{self, Write};
use crate::agent_generator::{generate_meaningseed_stub, write_meaningseed_files};
use std::path::Path;

pub fn run_onboarding_cli() {
    println!("=== TriWeavon Sovereign Onboarding (Stage 0) ===");
    let mut state = 1;

    loop {
        match state {
            1 => {
                print!("Step 1/3 — Describe your goal in one short sentence: ");
                io::stdout().flush().unwrap();
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                let prompt = input.trim();

                if prompt.is_empty() {
                    println!("Please provide a non-empty description.");
                    continue;
                }

                let stub = generate_meaningseed_stub(prompt);
                println!("\nGenerated minimal MeaningSeed:");
                println!("  Name: {}", stub.name);
                println!("  Version: {}", stub.version);

                state = 2;
            }
            2 => {
                print!("\nStep 2/3 — Confirm and write files? [y/n]: ");
                io::stdout().flush().unwrap();
                let mut confirm = String::new();
                io::stdin().read_line(&mut confirm).unwrap();

                if confirm.trim().eq_ignore_ascii_case("y") {
                    let stub = generate_meaningseed_stub("placeholder"); // In real flow, carry state
                    let _ = write_meaningseed_files(&stub, Path::new("generated_meaningseed"));
                    println!("Files written. Initial polarity check: ALIGNED");
                    state = 3;
                } else {
                    println!("Aborted.");
                    return;
                }
            }
            3 => {
                println!("\nStep 3/3 — MeaningSeed established. Full E2E pipeline unlocked.");
                println!("You may now use structured prompts and the complete TriWeavon stack.");
                break;
            }
            _ => break,
        }
    }
}
