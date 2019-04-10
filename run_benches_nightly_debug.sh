for file in ct-bencher/*[.rs];
    do  
        mkdir bench-results;
        filename=$(basename "$file");
        fname="${filename%.*}"; # Filename without .rs extension
        cargo +nightly run --no-default-features --features nightly --bin $fname > bench-results/$fname.txt;
    done