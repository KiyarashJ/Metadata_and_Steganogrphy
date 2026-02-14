# 🛡️ MetaStego – Your Ultimate Image Privacy & OSINT Powerhouse 🔍
Strip metadata, hide secrets, uncover hidden truths – all in one blazing-fast desktop app!
MetaStego is a super-fast, secure, and professional tool for working with image metadata and digital steganography.
Built with Rust + Dioxus, no heavy external dependencies, and ready for OSINT investigations and privacy protection! 🚀
✨ Key Features
## 🧹 Delete Metadata
Completely and losslessly remove hidden metadata (EXIF, IPTC, XMP, ICC, etc.) from images – perfect for safe uploads to social media!
Optional checkbox to keep a copy of the original file before stripping. 📸➡️🔒

## 🔍 Read Metadata
Extract all hidden image information and save it in clean, human-readable .txt files inside the metadatas folder – ideal for OSINT work (GPS location, camera model, date, editing software, and more). 🕵️‍♂️


## 🔐 Steganography – Hide Message
Hide secret messages inside images using the powerful AnyHide algorithm!
The app automatically generates and saves Secret Key, Public Key, and carrier code in a .txt file. 🤫


## 🔓 Steganography – Extract Message
Provide the carrier code, Secret Key, and Public Key to extract the hidden message and save it to a .txt file! 🗝️


## ⚡ Blazing-Fast Batch Processing
Process thousands of files at once – even several gigabytes in seconds! (Powered by Rayon parallelism)


🖼️ Supported Formats

# JPEG • PNG • WebP

<br>
🎥 Screenshots (Coming soon!)
  Upload Interface
  Delete Mode
  Read Metadata
  Hide Message
  Extract Message
Real screenshots coming soon – go take some cool shots and drop them here! 📸
🚀 How to Run

### Clone the repo

```Bash
git clone https://github.com/KiyarashJ/Metadata_and_Steganography
```

```
cd EXIF_TAMPER
```

## Build and run (Dioxus desktop)

```Rust
dx serve
```

## Or build release version

```Rust
dx build --release
```

<br>

## You need the Rust toolchain – easiest way🪛🪛

<br>

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

<br>

##  Built with PASSION❤️‍🔥

<br>
Rust + Dioxus for incredible performance
AnyHide for secure steganography
metastrip and custom parsing for complete metadata extraction
<br>
<br>

##  License⚖️

MIT License – feel free to use, modify, and distribute!

<br>

⭐ If you like it, give the repo a star!
<br>
Got questions? Open an issue or ask in Discussions. 💬<br>
MetaStego – Protect your privacy, uncover secrets. 🕵️‍♂️🔒✨