timekubesess(){
  start=`date +%s.%N`
  for i in $(seq 1 20); do
    export KUBECONFIG=$(kubesess -v arn:aws:eks:eu-north-1:392162365071:cluster/toca-days-staging-eu-north-1 context):$HOME/.kube/config;
    kubectl get nodes;
    export KUBECONFIG=$(kubesess -v docker-desktop context):$HOME/.kube/config;
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
    kubectx arn:aws:eks:eu-north-1:392162365071:cluster/toca-days-staging-eu-north-1 > /dev/null 2>&1;
    kubectl get nodes;
    kubectx docker-desktop > /dev/null 2>&1; 
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
