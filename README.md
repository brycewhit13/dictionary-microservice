# dictionary-microservice

## Description

This project is a microservice that acts as a combined dictionary and thesaurus. It takes an word as an input and return the definition, part of speech, synonyms, and other information. This is achieved through the use of the [Free Dictionary API](https://dictionaryapi.dev/). 

## Running the program

1) cd into the directory with `cd dictionary-microservice`
2) Run the program using `cargo run`
3) Go to [](http://localhost:8080)

### Push the image to DockerHub

1) login to docker using `docker login` and enter your credentials
2) `docker build -t dictionary-microservice .`
3) `docker tag dictionary-microservice:latest brycewhit13/dictionary-microservice:latest`
4) `docker push brycewhit13/dictionary-microservice:latest`

## Build and run your Docker Container

1) Build with: `docker build -t dictionary-microservice .`
2) Run with: `docker run -it --rm -p 8080:8080 dictionary-microservice`

## Running with minikube

1) Download the minikube command-line tool [here](https://minikube.sigs.k8s.io/docs/start/)
2) Start the minikube service with `minikube start`
3) Create an alias using `alias kubectl="minikube kubectl --"`
4) Create a deployment: `kubectl create deployment dictionary --image=registry.hub.docker.com/brycewhit13/dictionary-microservice`
5) Create a service and expose it: `kubectl expose deployment dictionary --type=NodePort --port=8080`
6) Foward the port to the local 7080: `kubectl port-forward service/dictionary 7080:8080` and go to [](http://localhost:7080)
7) Delete the deployment for cleanup: `kubectl delete deployment dictionary`
8) Delete the service for cleanup: `kubectl delete service dictionary`
9) Stop the minikube service: `minikube stop`
