# About

This project aims to solve [Rosalind's](https://rosalind.info/about/) problems and package them in a command-line interface (CLI) format.

## Repo structure

```
rosalind-cli/  
├── Cargo.toml           
├── scripts/  
├── src/  
│   ├── main.rs           
│   ├── lib.rs          
│   └── problems/       
│       ├── mod.rs      
│       ├── prob1/  
|       |    ├── mod.rs  
|       |    ├── args.rs  
|       |    ├── solve.rs    
│       ├── prob2/     
│       └── ...         
└── README.md
```

## Structure of problem solving

- `solve.rs`: It contains the code to solve the problems.

- `mod.rs`: It connects the args and solve modules to the problem folder, organizing internal references and making them accessible to the parent module.

- `args.rs`: It defines the command-line arguments specific to the problem, usually using `clap` to parse and validate user inputs.

### Automatic problem structure generator

To make it easier to create new problems while maintaining the project's structure, the `add_problems.rs` script was designed. It checks the name of the problem to be created and automatically creates the (blank) files in the folder with the name of the problem.

Usage:

`rustc scripts/add_problem.rs -o add_problem #check the path!`

`./add_problem PROB`

Result:

```
src/problems/PROB/  
├── args.rs  
├── mod.rs   
└── solve.rs  
```

## Problems solved

- [DNA](https://rosalind.info/problems/dna/)
- [RNA](https://rosalind.info/problems/rna/)
- [REVC](https://rosalind.info/problems/revc/)
- [GC](https://rosalind.info/problems/gc/)
- [PROT](https://rosalind.info/problems/prot/)
- [SUBS](https://rosalind.info/problems/subs/)
- [HAMM](https://rosalind.info/problems/hamm/)
- [FIB](https://rosalind.info/problems/fib/)
- [IPRB](https://rosalind.info/problems/iprb/)
- [IEV](https://rosalind.info/problems/iev/)

## Get in touch

<div align = "center">
<a href = "mailto:marcel.ferreira@unesp.br"><img src="https://upload.wikimedia.org/wikipedia/commons/thumb/7/7e/Gmail_icon_%282020%29.svg/2560px-Gmail_icon_%282020%29.svg.png" title="Gmail" alt="Gmail" width="45" height="40"/></a>&nbsp;
<a href="https://www.linkedin.com/in/marceelrf/"><img src="https://github.com/devicons/devicon/blob/master/icons/linkedin/linkedin-original.svg" title="LinkedIn" alt="LinkedIn" width="40" height="40"/></a>&nbsp;
<a href="https://github.com/marceelrf"><img src="https://github.com/devicons/devicon/blob/master/icons/github/github-original.svg" title="Github" alt="Github" width="40" height="40"/></a>&nbsp;
</div>
