# FONTS

Many terminal themes, such as `agnoster`, recommend using patched Nerd Fonts like **MesloLGS NF** to properly render Powerline symbols and icons.

---

## Powerline Fonts

### Installation

#### Option 1: Install via APT (for Ubuntu/Debian)
```bash
sudo apt-get install fonts-powerline
```

#### Option 2: Manual Installation from GitHub
```bash
cd ~
git clone https://github.com/powerline/fonts.git --depth=1
cd fonts
./install.sh
cd ..
rm -rf fonts
```

---

### Verifying Installation

To confirm that Powerline fonts are installed:
```bash
fc-list | grep -i powerline
```

Expected output:
```bash
/usr/share/fonts/truetype/powerline-symbols/PowerlineSymbols.otf: PowerlineSymbols:style=Regular
```

---

### Configuring Terminal to Use Powerline-Compatible Fonts

1. Open your terminal's **Preferences**
2. Select your active **Profile**
3. Navigate to the **Text** or **Font** settings
4. Enable **Custom Font**
5. Choose a Nerd Font such as:
   - MesloLGS NF
   - Fira Code Nerd Font
   - Hack Nerd Font

Ensure the selected font supports Powerline symbols.

---

### Testing Font Rendering

Run the following to test Powerline glyph rendering:
```bash
echo "\uE0B0 \uE0B1 \uE0B2 \uE0B3 \uE0A0 \uE0A1 \uE0A2"
```

If configured correctly, this should display a series of special symbols. If you see boxes or question marks instead, the font is either missing or not active in your terminal.

