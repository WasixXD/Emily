# Emily 


# Brief 📖
A simple TODOs tracker that find unreported todos and submit them as an issue on the repo

# Challenges 🐢
- Find TODOs via multiple archives and directorys
- Report them to github without repetition

# Goals 🏆
[ x ] User can see TODOs without reporting them to github\
[ x ] Script create a issue with the file name and TODOs description\
[ x ] Issues doesn't repeat\
[ x ] Work from 

# How it works? 💼
The app receive the directory path and start searching for TODOs index through the files, them if the --push is activated it get all issues on the repo and report the unrepetead ones on github

# How to install 🚀
Have sure to have cargo on your terminal and run 

```
$ cargo install Emily
```

# How to use :construction_worker:
## TODOs
The todo have 2 main things to work: 
- the <strong> # </strong> sign
- a new line under the TODO 

```JS
1 console.log("Hello Reality");
2 // #TODO Use World instead of Reality
3  
```
## The command line
The counts with the following flags:
- --dir (The dir path) [Required]
- --name (Your github name) [Only required if you will report the TODOs]
- --repo (The repository that the TODOs will be reported [Only required if you will report the TODOs]
- --key (the github api key / see how to get one : https://securitysenses.com/videos/obtaining-api-key-github-api [Only required if you will report the TODOs]
- --push (this determine if the app will make the reports or will only show them in terminal) [Only required if you will report the TODOs]


# DEMO
- Without push
<img src="https://github.com/WasixXD/Emily/blob/master/Emily1.png?raw=true">

- Pushing to github <br>
*NOTE*: $GITHUB_KEY = subtitute with your api key
<img src="https://github.com/WasixXD/Emily/blob/master/Emily2.png?raw=true">

<img src="https://github.com/WasixXD/Emily/blob/master/Emily3.png?raw=true">


