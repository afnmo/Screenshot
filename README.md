# Screenshot Capture

This Rust program captures a screenshot, processes it into a grayscale flipped image, saves it as a JPEG, and then compresses it into a ZIP file.

---

# Features
- Captures a screenshot using the macOS screencapture command.

- Converts the screenshot to grayscale.

- Flips the image vertically.

- Saves the processed image as a JPEG (output.jpg).

- Compresses the JPEG into a ZIP archive (screenshot.zip).

---
# Requirements
- Rust (latest stable version recommended)

- macOS (uses the screencapture command)

- Dependencies in Cargo.toml:

```toml
[dependencies]
image = "0.24"
zip = "0.6"
```


---

# How to Run
1. Clone or download this project.

2. Ensure you have Rust installed. If not, install via rustup.

3. Add the required dependencies to your Cargo.toml.

4. Build and run the program:

```bash
cargo run
```
After running:

- A screenshot will be captured and saved temporarily as ```temp.png```.

- The processed grayscale flipped image will be saved as ```output.jpg```.

- The image will be compressed into ```screenshot.zip```.

---

# Output
```output.jpg``` → Grayscale, vertically flipped screenshot.

```screenshot.zip``` → Contains the processed image.
