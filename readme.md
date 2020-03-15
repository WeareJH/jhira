# jhira ![](https://github.com/WeareJH/jhira/workflows/.github/workflows/test.yml/badge.svg)

## Express Install

Simply run the following in your terminal

```
zsh <(curl -L https://raw.githubusercontent.com/WeareJH/jhira/master/express-install.sh) && source ~/.zshrc
```

## Install
`jhira` is distributed as a single binary with everything packaged inside -
this means you *do not* need PHP or Composer installed on your machine.

1. Download the latest version from the [releases page](https://github.com/WeareJH/jhira/releases)
2. Make the file executable: (assuming it downloaded to the `Downloads` folder)

    `chmod +x ~/Downloads/jhira`
    
3. Move the executable from your Downloads folder to /opt

    `sudo mv ~/Downloads/jhira /opt`
    
    - If "opt" does not exist run the command below

        `sudo mkdir /opt`
    
    - Then make sure the permissions are correct on the folder
    
        `sudo chown -R $(whoami) /opt`

5. Add this to the bottom of your *zshrc* or *bash_profile*:

    `export PATH="$PATH:/opt"`

6. Use the following command to refresh any already open terminals

    `source ~/.zshrc`

7. Or for bash users

    `source ~/.bash_profile`

8. Type the following command to check all is installed OK:

    `jhira`

9. You should see the same output as below (in features):

## Help

For help on the commands available, run the following: 

```shell script
jhira --help
```

`--help` is recipe specific. So if you're in a M2 project, you'll only see
commands that are relevant to M2 sites.

## Contributing.

Before pushing any code, run the following to ensure you catch
problems before they get to CI

```shell script
bash pre-push.sh
```
