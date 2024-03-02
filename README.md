# Natural Computing

This repository contains an implementation of Cellular Automata and Cellular Potts Models. The project `act-cpm-ui`, when executed, opens a window with an Act-CPM simulation.

## Usage
### Command-line interface
```
Usage: act-cpm-ui.exe [OPTIONS]

Options:
  -o, --output <DIR>               Path to the output directory. Also see --save-interval
      --save-interval <ITER>       Interval (in simulation steps) in which to export images. Also specify --output
  -i, --iter <ITER>                Stop after this many simulation steps
      --cell-grid <HOR_RES>        Number of cells in one direction of the grid (total = cell-grid²) [default: 13]
      --obstacle-grid <HOR_RES>    Number of obstacles in one direction of the grid (total = obstacle-grid²) [default: 5]
      --temp <TEMP>                Simulation temperature [default: 20]
      --l-adhesion <L_ADHESION>    λ adhesion [default: 20]
      --volume <PIXELS>            Target volume in number of pixels [default: 200]
      --l-volume <L_VOLUME>        λ volume [default: 50]
      --perimeter <EDGES>          Target perimeter in number of pixel edges [default: 180]
      --l-perimeter <L_PERIMETER>  λ perimeter [default: 2]
      --max-act <MAX_ACT>          Max act value [default: 80]
      --l-act <L_ACT>              λ-act [default: 300]
  -v, --verbose                    Log frame times
  -h, --help                       Print help
  -V, --version                    Print version
```

### Key bindings
- **Space:** toggle pause
- **Arrow Down:** speed / 2
- **Arrow Up:** speed * 2
- **Arrow Right:** single step (when paused)
- **Ctrl+S:** save image (also see the `--save-interval` command-line parameter)
