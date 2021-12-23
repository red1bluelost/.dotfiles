#!/bin/bash
set -o errexit

link() {
  echo "Linking $1 ..."
  rm -f ~/$1 && ln -s ~/.dotfiles/$1 ~/$1
}

link .gitconfig
link .bashrc

