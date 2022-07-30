timekubesess(){
  start=`date +%s.%N`
  for i in $(seq 1 20); do
    export KUBECONFIG=$(~/kubesess/kubesess -- context other-cluster):$HOME/.kube/config;
    # k get nodes;
    export KUBECONFIG=$(~/kubesess/kubesess -- context docker-desktop):$HOME/.kube/config;
    # k get nodes;
  done
  end=`date +%s.%N`
  echo time
  runtime=$( echo "$end - $start" | bc -l )
  echo $runtime
}

timekubectx(){
  start=`date +%s.%N`
  for i in $(seq 1 20); do 
    kubectx other-cluster > /dev/null 2>&1;
    # k get nodes;
    kubectx docker-desktop > /dev/null 2>&1; 
    # k get nodes;
  done
  end=`date +%s.%N`
  echo time
  runtime=$( echo "$end - $start" | bc -l )
  echo $runtime
}
