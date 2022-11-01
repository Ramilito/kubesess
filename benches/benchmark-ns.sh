#!/usr/bin/env bash

timekubesess(){
  start=`date +%s.%N`
  for i in $(seq 1 20); do
    export KUBECONFIG=$(kubesess -v back-end namespace):$HOME/.kube/config;
    export KUBECONFIG=$(kubesess -v monitoring namespace):$HOME/.kube/config;
  done
  end=`date +%s.%N`
  echo time
  runtime=$( echo "$end - $start" | bc -l )
  echo $runtime
}

timekubectx(){
  start=`date +%s.%N`
  for i in $(seq 1 20); do 
    kubens back-end > /dev/null 2>&1;
    kubens monitoring > /dev/null 2>&1; 
  done
  end=`date +%s.%N`
  echo time
  runtime=$( echo "$end - $start" | bc -l )
  echo $runtime
}


echo "Starting kubesess test:"
timekubesess
echo "Ending kubesess test:"

echo "Starting kubectx test:"
timekubectx
echo "Ending kubectx test:"
