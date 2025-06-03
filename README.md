# Windows Key State Checker



https://github.com/user-attachments/assets/aa6d2580-8617-48b4-8e69-697e1db27071

Download on the releases page, or click [here](https://github.com/TeamDman/input-debug/releases/download/v0.1.0/input-debug.exe)



A simple Rust command-line utility that continuously monitors and displays all keyboard keys currently reported as pressed by the Windows operating system. This tool is useful for diagnosing issues where the OS might incorrectly believe a key (like Ctrl, Shift, or Alt) is stuck or being held down.

## Features

*   **Real-time Monitoring:** Continuously polls the state of all virtual keys.
*   **Detects Pre-pressed Keys:** Correctly identifies keys that were already held down before the program started, as it queries the current OS state rather than relying on key-down events.
*   **Human-Readable Output:** Displays the names of pressed keys (e.g., "Left Ctrl", "A", "Space").
*   **Low-Level Access:** Uses the `windows-rs` crate to directly call the Windows API function `GetAsyncKeyState`.
*   **Lightweight:** Minimal dependencies and efficient polling.

## Why This Tool?

Sometimes, you might encounter strange behavior in applications or the OS itself, such as:
*   Unexpected shortcuts triggering.
*   Typing producing different characters than expected.
*   UI elements behaving as if a modifier key is constantly active.

This tool helps identify if the operating system *thinks* a key is pressed, which is the first step in diagnosing such "phantom key press" problems.

## Usage

Once running, the program will continuously print a list of all keys currently detected as pressed.

*   If no keys are pressed, it will display: `No keys pressed.`
*   If one or more keys are pressed, it will display: `Pressed: Key1, Key2, ...`

The output updates approximately 20 times per second.

To **exit** the program, press `Ctrl+C` in the terminal where it's running.

## Technical Details

The core of this utility relies on the `GetAsyncKeyState` function from the Windows API (`Win32_UI_Input_KeyboardAndMouse`). This function returns the status of the specified virtual key at the time the function is called.

*   The program iterates through virtual key codes 1 to 254.
*   For each key code, it checks if the most significant bit of the value returned by `GetAsyncKeyState` is set. If it is, the key is considered to be currently down.
*   A mapping function (`vk_to_string`) converts the virtual key codes to human-readable names.
