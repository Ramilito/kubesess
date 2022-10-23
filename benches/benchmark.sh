#!/usr/bin/env bash

timekubesess(){
  start=`date +%s.%N`
  for i in $(seq 1 20); do
    export KUBESESSCONFIG=$(kubesess -v docker-desktop context):$HOME/.kube/config;
    kubectl get nodes;
    export KUBESESSCONFIG=$(kubesess -v docker-desktop-2 context):$HOME/.kube/config;
    kubectl get nodes;
  done
  end=`date +%s.%N`
  echo time
  runtime=$( echo "$end - $start" | bc -l )
  echo $runtime
}

timekubectx(){
  start=`date +%s.%N`
  for i in $(seq 1 20); do 
    kubectx docker-desktop > /dev/null 2>&1;
    kubectl get nodes;
    kubectx docker-desktop-2 > /dev/null 2>&1; 
    kubectl get nodes;
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
