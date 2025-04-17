
## Workspace Grid

### Set Workspace grid 
```
# Install dconf
sudo apt install dconf-editor

# Set the grid size
WNUM="3

# Number of vertical workspaces
dconf write /org/compiz/profiles/unity-lowgfx/plugins/core/vsize ${WNUM}

# Number of horizontal workspaces
dconf write /org/compiz/profiles/unity-lowgfx/plugins/core/hsize ${WNUM}
```

### Verify
```
dconf read /org/compiz/profiles/unity-lowgfx/plugins/core/vsize
dconf read /org/compiz/profiles/unity-lowgfx/plugins/core/hsize
```