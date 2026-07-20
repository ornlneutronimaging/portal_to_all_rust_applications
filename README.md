# Portal to all Rust applications

Desktop GUI (Rust / egui, same template as the other neutron imaging tools —
imaging logo top right) that lists every Rust application available in
`/SNS/VENUS/shared/software/git` and launches the one you select.

## Usage

```bash
./launch_portal.sh
```

The script rebuilds the portal automatically if the sources changed, then
starts it. Requires a graphical session (e.g. ThinLinc).

Each application card shows a short description and a **Launch** button. The
button is disabled (with an explanatory tooltip) if the application's launch
script or release binary cannot be found.

## Applications listed

| Application | Repository | Started via |
|---|---|---|
| TIFF Viewer | `rust_tiff_viewer` | `launch_rust_tiff_viewer.sh` |
| TOF Profile Viewer | `rust_tof_profile_viewer` | `launch_tof_profile_viewer.sh` |
| ROI Selector | `rust_roi_selector` | `launch_roi_selector.sh` |
| CT Reconstruction | `rust_ct_reconstruction` | `launch_ct_reconstruction.sh` |
| Hyperspectral Masker | `rust_hyperspectra_masker` | `launch_rust_hyperspectral_makser.sh` |
| Auto Normalization Monitor | `rust_autonormalization_monitor` | `launch_autonormalization_monitor.sh` |
| Jupyter Notebooks Portal | `rust_jupyter_notebooks_portal` | release binary |
| Jupyter Portal (dedicated IPTS) | `rust_jupyter_portal_dedicated_ipts` | release binary |
| Marimo Portal (general tools) | `rust_marimo_portal_general_tools` | release binary |
| Marimo Portal (dedicated IPTS) | `rust_marimo_portal_dedicated_ipts` | release binary |

## Adding a new application

Edit the `APPS` table at the top of `src/main.rs` and add an entry with the
application name, a one-line description, its repository folder name, and how
it is launched (`Launcher::Script("launch_xxx.sh")` for repos with a launch
script, `Launcher::Binary("binary_name")` for a `target/release` binary).
