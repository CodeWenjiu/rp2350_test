#!/usr/bin/env nu

def help [] {
    print "Usage: run.nu [options]"
    print "Options:"
    print "  -h, --help      Display this help message"
}

def list_probes [] {
    print "Listing probes..."
    probe-rs list
}

def debug [bin] {
    print "Build and Flash in Debug Mode..."
    cargo run --bin $bin
}

def run [bin] {
    print "Build and Flash in Release Mode..."
    cargo run --release --bin $bin
}

def clean [] {
    print "Cleaning project..."
    cargo clean
}

def main [...args] {
    if ($args | is-empty) {
        help
        return
    }

    let command = $args.0?

    match $command {
        "help" => help
        "list" => list_probes
        "debug" => {
            if ($args | length) >= 2 {
                debug $args.1
            } else {
                print "Error: debug command requires a binary name"
                print "Usage: debug <bin>"
            }
        }
        "run" => {
            if ($args | length) >= 2 {
                run $args.1
            } else {
                print "Error: run command requires a binary name"
                print "Usage: run <bin>"
            }
        }
        "clean" => clean
        _ => {
            print "Unknown command, Script Usage:"
            help
        }
    }
}
