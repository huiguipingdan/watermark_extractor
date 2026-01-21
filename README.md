# Watermark Extractor
A lightweight, cross-platform command-line tool for extracting watermarks from images with zero external dependencies. This tool provides native support for **Windows** (executable) and **macOS** (universal binary for Intel & Apple Silicon architectures), enabling fast and hassle-free watermark extraction with minimal setup.

## Features
- 🖥️ **Cross-Platform Compatibility**: Runs natively on Windows and macOS without requiring runtime environments like Python or Rust.
- 🚀 **Zero Dependencies**: Standalone executable files—no extra libraries or installations needed.
- ⚡ **Lightning-Fast Processing**: Optimized for speed to handle common image formats (JPG, PNG, etc.) efficiently.
- 🎯 **Intuitive Usage**: Two simple operation modes tailored to each OS’s user habits.

<img width="2474" height="1546" alt="cd64892e-ae2a-4f02-87de-c67259a61896" src="https://github.com/user-attachments/assets/fbbc93cc-7505-4559-902b-65ce345dee3b" />


## Installation
1. Go to the [Releases](https://github.com/huiguipingdan/watermark_extractor/releases) page of this repository.
2. Download the corresponding package for your operating system:
   - Windows: `win-binary.zip`
   - macOS: `mac-binary.zip`
3. Extract the downloaded file to a directory of your choice.

## Usage
### For macOS Users
> **Note**: macOS may block unsigned apps by default—follow these steps to bypass the security prompt.

1. After extracting the package, **do NOT double-click** the executable directly if you see the prompt: *"Cannot verify developer"*.
2. Right-click the `watermark_extractor_mac_universal` file → select **Open**.
3. Click **Open** again in the pop-up confirmation window to grant permission.
4. Launch **Terminal** and navigate to the directory where the executable is located:
   ```bash
   cd /Users/your-username/path-to-extracted-folder
   ```
5. Run the tool with your target image file:
   ```bash
   ./watermark_extractor_mac_universal your-watermark-image.jpg
   ```

### For Windows Users
The Windows version supports a **drag-and-drop** workflow for maximum convenience:
1. Locate the extracted `watermark_extractor.exe` file.
2. Simply drag and drop your watermark-containing image file **directly onto the EXE file**.
3. The tool will automatically process the image and output the extracted watermark.

## FAQ
- **Q: Why does macOS block the executable?**
  A: This is a default macOS security feature for unsigned apps. The right-click → Open method safely bypasses this restriction without compromising your system.
- **Q: What image formats are supported?**
  A: Currently, the tool supports common formats including JPG, PNG.
- **Q: Where is the extracted watermark saved?**
  A: The extracted watermark file will be saved in the **same directory** as your input image, with the suffix `_processed` (e.g., `your-image_processed.png`).

