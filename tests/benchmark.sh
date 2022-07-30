timekubesess(){
  start=`date +%s.%N`
  for i in $(seq 1 10); do
    ~/kubesess/kubesess docker-desktop; 
    kgs;
    ~/kubesess/kubesess test-context-on-aws; 
    kgs;
  done
  end=`date +%s.%N`
  echo time
  runtime=$( echo "$end - $start" | bc -l )
  echo $runtime
}

timekubectx(){
  start=`date +%s.%N`
  for i in $(seq 1 10); do 
    kubectx docker-desktop; 
    kgs;
    ~/kubesess/kubesess test-context-on-aws; 
    kgs;

  done
  end=`date +%s.%N`
  echo time
  runtime=$( echo "$end - $start" | bc -l )
  echo $runtime
}
