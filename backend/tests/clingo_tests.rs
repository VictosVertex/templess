#[test]
fn test_solve_iterator_pattern() -> Result<(), Box<dyn std::error::Error>> {
    use std::path::PathBuf;
    use templess::clingo::configuration::ConfigKey;
    use templess::clingo::control::Control;

    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let instance_lp = manifest_dir.join("..").join("instance.lp");
    let encoding_lp = manifest_dir
        .join("src")
        .join("optimization")
        .join("encoding.lp");

    // 1. Setup Control
    let mut control = Control::new()?;
    control
        .configuration_mut()?
        .set_key_to_value(ConfigKey::SolveModels, "0")?;
    control.load(instance_lp.to_string_lossy().as_ref())?;
    control.load(encoding_lp.to_string_lossy().as_ref())?;
    control.ground()?;

    println!("--- Starting Iterator Solve ---");

    // 2. Start Solve in YIELD mode (1 = clingo_solve_mode_yield)
    // This returns immediately.
    let mut handle = control.solve()?;

    let mut model_count = 0;

    // 3. Manual Drive Loop
    loop {
        // A. WAIT for the solver to reach a stopping point (Model Found or Exhausted)
        // Since we used Async, we must explicitly wait here.
        handle.get()?;

        // B. Check if we have a model
        if let Some(model) = handle.model()? {
            model_count += 1;

            let symbols = match model.symbols(2) {
                Ok(s) => s,
                Err(e) => {
                    println!("Error retrieving symbols: {}", e);
                    break;
                }
            };

            for symbol in symbols {
                println!("kind: {:?} name: {}", symbol.kind(), symbol.to_string());

                let arguments = match symbol.arguments() {
                    Ok(args) => args,
                    Err(e) => {
                        println!("Error retrieving symbols: {}", e);
                        break;
                    }
                };

                for arg in arguments {
                    println!("Arg: kind: {:?} name: {}", arg.kind(), arg.to_string());
                }
            }

            if model_count > 3 {
                break;
            }

            // C. Trigger the search for the NEXT model
            // We only resume if we found a model and want another one.
            handle.resume()?;
        } else {
            // No model returned means the search is complete/exhausted.
            println!("--- Search Finished ---");
            break;
        }
    }

    // 4. Verify results
    assert_eq!(model_count, 4);

    // Check final status (should be Satisfiable + Exhausted)
    let result = handle.get()?;
    println!("Final Result: {:?}", result);

    Ok(())
}

#[test]
fn test_live_ui_updates_pattern() -> Result<(), Box<dyn std::error::Error>> {
    use std::path::PathBuf;
    use std::sync::mpsc::channel;
    use std::thread;
    use std::time::Duration;
    use templess::clingo::configuration::ConfigKey;
    use templess::clingo::control::Control;

    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let test_lp = manifest_dir.join("..").join("test.lp");

    // 1. Setup Control (Main Thread)
    let mut control = Control::new()?;
    control
        .configuration_mut()?
        .set_key_to_value(ConfigKey::SolveModels, "0")?;

    // Use load() instead of add()
    // Make sure "test.lp" exists and contains something like "1 { a; b; c } 1."
    control.load(test_lp.to_string_lossy().as_ref())?;
    control.ground()?;

    // 2. Setup Channels
    let (tx, rx) = channel();

    println!("--- [Main] Spawning Worker Thread ---");

    // 3. Spawn Worker Thread
    // The 'move' keyword transfers ownership of 'control' to the new thread.
    // This requires 'unsafe impl Send for Control {}' to work!
    thread::spawn(move || {
        // A. Start Solve (Async + Yield = 3)
        // 1 (Yield) | 2 (Async) = 3
        let mut handle = control.solve().expect("Solve failed");

        loop {
            // B. Wait for next signal (Blocks ONLY this worker thread)
            handle.get().expect("Get failed");

            // C. Check if a model is ready
            if let Some(model) = handle.model().expect("Model failed") {
                // In a real app, you would parse the model here.
                // We just send a signal for the test.
                tx.send(format!(
                    "Model found on thread {:?}",
                    thread::current().id()
                ))
                .unwrap();

                // Slow down slightly so we can see the UI thread print in between
                thread::sleep(Duration::from_millis(50));

                // D. Resume search
                handle.resume().expect("Resume failed");
            } else {
                tx.send("Done".to_string()).unwrap();
                break;
            }
        }
    });

    println!("--- [Main] Worker Spawned. I am free! ---");

    // 4. The "UI Loop" (Main Thread)
    // This proves the main thread is NOT blocked.
    loop {
        // Non-blocking check for messages
        match rx.try_recv() {
            Ok(msg) => {
                println!("    [UI Update] {}", msg);
                if msg == "Done" {
                    break;
                }
            }
            Err(std::sync::mpsc::TryRecvError::Empty) => {
                // No message? Print "Idle" to prove we are running!
                println!("[UI Loop] ... Main thread is responsive ...");
                thread::sleep(Duration::from_millis(20));
            }
            Err(_) => break, // Channel closed
        }
    }

    Ok(())
}
