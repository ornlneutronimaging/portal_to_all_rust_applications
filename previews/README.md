# Application previews

Drop one screenshot per application in this directory and the portal shows it
in the right-hand preview panel when the mouse hovers over that application.
Images are loaded at runtime — no rebuild needed, just restart the portal.

Accepted formats: `.png`, `.jpg`, `.jpeg` (checked in that order).

Expected file names (the `preview` key in `src/main.rs`):

| Application | File name |
| --- | --- |
| TIFF Viewer | `rust_tiff_viewer.png` |
| TOF Profile Viewer | `rust_tof_profile_viewer.png` |
| ROI Selector | `rust_roi_selector.png` |
| Crop TIFF | `rust_crop_tiff.png` |
| CT Reconstruction | `rust_ct_reconstruction.png` |
| Hyperspectral Masker | `rust_hyperspectra_masker.png` |
| Auto Normalization Monitor | `rust_autonormalization_monitor.png` |
| BM3D ORNL GUI | `rust_bm3dornl.png` |
| NRAD Spot Cleaner | `nrad_spot_cleaner.png` |
| Timepix3 Tool | `rustpix.png` |
| Jupyter Notebooks Portal | `rust_jupyter_notebooks_portal.png` |
| Jupyter Portal (dedicated IPTS) | `rust_jupyter_portal_dedicated_ipts.png` |
| Marimo Portal (general tools) | `rust_marimo_portal_general_tools.png` |
| Marimo Portal (dedicated IPTS) | `rust_marimo_portal_dedicated_ipts.png` |

Missing files simply show "No preview available" in the panel.
