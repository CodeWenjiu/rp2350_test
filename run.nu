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

def debug [] {
    print "Build and Flash in Debug Mode..."
    cargo run
}

def run [] {
    print "Build and Flash in Release Mode..."
    cargo run --release
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

    match $args {
        ["help"] => help
        ["list"] => list_probes
        ["debug"] => debug
        ["run"] => run
        ["clean"] => clean
        _ => {
            print "Unknown command, Script Usage:"
            help
        }
    }
}
