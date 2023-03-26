from diagrams import Cluster, Diagram, Edge

from diagrams.k8s.clusterconfig import HPA
from diagrams.k8s.compute import Deployment, Pod, ReplicaSet, StatefulSet, Job, DaemonSet
from diagrams.k8s.network import Ingress, Service
from diagrams.k8s.storage import PV, PVC
from diagrams.k8s.podconfig import ConfigMap, Secret
from diagrams.k8s.rbac import ClusterRole, ClusterRoleBinding, ServiceAccount
from diagrams.k8s.infra import Node

from diagrams.onprem.queue import RabbitMQ
from diagrams.onprem.compute import Server
from diagrams.onprem.network import Traefik
from diagrams.onprem.database import PostgreSQL, MongoDB
from diagrams.onprem.inmemory import Redis
from diagrams.onprem.monitoring import Grafana, Prometheus
from diagrams.onprem.aggregator import Fluentd
from diagrams.onprem.client import User

from diagrams.elastic.elasticsearch import Elasticsearch, Kibana


graph_attr = {
    "bgcolor": "transparent"
}

# Minerva
with Diagram("Arquitetura do Minerva System", show=False, outformat="png", graph_attr=graph_attr, filename="minerva_diagram"):
    with Cluster("Ingress"):
        ingress = Traefik("Traefik")

    with Cluster("Services"):
        rest = Server("REST")
        users = Server("USERS")
        session = Server("SESSION")

    with Cluster("Databases"):
        with Cluster("PostgreSQL"):
            postgres = PostgreSQL("postgres")
        with Cluster("MongoDB"):
            mongo = MongoDB("mongo")
        with Cluster("Redis"):
            redis = Redis("master")
            replicas = [Redis("replica")]
            redis - Edge(color="brown", style="dashed") - replicas

    with Cluster("Messaging"):
        dispatch = Server("DISPATCH")
        rabbitmq = RabbitMQ("rabbitmq")
        
    with Cluster("Monitoring"):
        prometheus = Prometheus("prometheus")
        elastic = Elasticsearch("elasticsearch")
        grafana = Grafana("grafana")
        kibana = Kibana("kibana")
        with Cluster("Logging (per K8s node)"):
            node = Node("node")
            fluentd = Fluentd("fluentd")

    # Ingress
    ingress >> Edge(color="lightblue") >> rest
    ingress >> Edge(color="lightblue") >> grafana
    ingress >> Edge(color="lightblue") >> kibana
    
    # Services dispatch
    rest >> Edge(color="black") << users
    rest >> Edge(color="black") << session
    dispatch >> Edge(color="black") >> session

    # Messaging
    dispatch >> Edge(color="darkorange") << rabbitmq
    session >> Edge(color="darkorange", style="dashed") >> rabbitmq

    # Log management
    node >> Edge(color="darkblue", label="Pod logs") >> fluentd >> Edge(color="darkblue", style="dashed") >> elastic

    # Cache
    session >> Edge(color="brown", label="Session cache") << redis

    # Databases
    session >> Edge(color="darkgreen", label="Session data") << mongo
    users >> Edge(color="blue") << postgres
    session >> Edge(color="blue") << postgres

    # Metrics
    prometheus << Edge(color="brown") << rabbitmq
    grafana >> Edge(color="darkorange", style="dashed") >> [prometheus, elastic]
    kibana >> Edge(color="magenta", style="dashed") >> elastic
    
# REST
with Diagram("Provisionamento do serviço REST", show=False, outformat="png", graph_attr=graph_attr, filename="rest_diagram"):
    ports = ConfigMap("ports-configmap")
    servers = ConfigMap("servers-configmap")
    ingress = Ingress("/api")
    with Cluster("REST"):
        service = Service("rest-svc\n(LoadBalancer)")
        deployment = Deployment("rest-deployment")
        configmap = ConfigMap("rest-configmap")
        hpa = HPA("rest-hpa")

        ingress >> service >> deployment
        hpa >> deployment
        with Cluster("ReplicaSet"):
            pods = [Pod("rest")]
            deployment - Edge(style="dashed") - pods
            pods - configmap
            pods - ports
            pods - servers


# Dispatch
with Diagram("Provisionamento do serviço DISPATCH", show=False, outformat="png", graph_attr=graph_attr, filename="dispatch_diagram"):
    ports = ConfigMap("ports-configmap")
    servers = ConfigMap("servers-configmap")
        
    with Cluster("DISPATCH"):
        deployment = Deployment("dispatch-deployment")
        hpa = HPA("dispatch-hpa")
        hpa >> deployment
        with Cluster("ReplicaSet"):
            pods = [Pod("dispatch")]
            deployment - Edge(style="dashed") - pods
            pods - ports
            pods - servers


# User
with Diagram("Provisionamento do serviço USER", show=False, outformat="png", graph_attr=graph_attr, filename="user_diagram"):
    ports = ConfigMap("ports-configmap")
    servers = ConfigMap("servers-configmap")
    with Cluster("USER"):
        service = Service("user-svc\n(ClusterIP)")
        deployment = Deployment("user-deployment")
        hpa = HPA("user-hpa")

        service >> deployment
        hpa >> deployment
        with Cluster("ReplicaSet"):
            pods = [Pod("user")]
            deployment - Edge(style="dashed") - pods
            pods - ports
            pods - servers

# Session
with Diagram("Provisionamento do serviço SESSION", show=False, outformat="png", graph_attr=graph_attr, filename="session_diagram"):
    ports = ConfigMap("ports-configmap")
    servers = ConfigMap("servers-configmap")
    with Cluster("SESSION"):
        service = Service("session-svc\n(ClusterIP)")
        deployment = Deployment("session-deployment")
        hpa = HPA("session-hpa")

        service >> deployment
        hpa >> deployment
        with Cluster("ReplicaSet"):
            pods = [Pod("session")]
            deployment - Edge(style="dashed") - pods
            pods - ports
            pods - servers


# PostgreSQL
with Diagram("Provisionamento do banco de dados PostgreSQL", show=False, outformat="png", graph_attr=graph_attr, filename="postgresql_diagram"):
    with Cluster("PostgreSQL"):
        secret = Secret("postgresql-secret")
        pvc = PVC("postgresql-pvc\n(1Gi)")
        deployment = Deployment("postgresql-deployment")
        service = Service("postgresql-svc\n(ClusterIP)")
        volume = PV("1Gi volume\n(dynamic)")
        
        service >> deployment
        deployment << pvc
        with Cluster("ReplicaSet"):
            pods = [Pod("postgresql")]
            deployment - Edge(style="dashed") - pods
            pods - secret
            pods - Edge(style="dotted") - volume
    

# MongoDB
with Diagram("Provisionamento do banco de dados MongoDB", show=False, outformat="png", graph_attr=graph_attr, filename="mongodb_diagram"):
    with Cluster("MongoDB"):
        secret = Secret("mongodb-secret")
        pvc = PVC("mongodb-pvc\n(1Gi)")
        deployment = Deployment("mongodb-deployment")
        service = Service("mongodb-svc\n(ClusterIP)")
        volume = PV("1Gi volume\n(dynamic)")

        service >> deployment
        deployment << pvc
        with Cluster("ReplicaSet"):
            pods = [Pod("mongodb")]
            deployment - Edge(style="dashed") - pods
            pods - secret
            pods - Edge(style="dotted") - volume

# Redis
with Diagram("Provisionamento do banco de dados in-memory Redis", show=False, outformat="png", graph_attr=graph_attr, filename="redis_diagram"):
    with Cluster("Redis"):
        configmap = ConfigMap("redis-configmap\n(mounted as volume)")
        statefulset = StatefulSet("redis")
        service = Service("redis-svc\n(ClusterIP)")
        hpa = HPA("redis-hpa")

        service >> statefulset << hpa
        with Cluster("ReplicaSet"):
            pod0 = Pod("redis-0\n(master)")
            pod1 = Pod("redis-1")
            pods = [pod0, pod1]
            #pod0 - Edge(style="dotted") - PVC("redis-data-redis-0\n(dynamic)") - Edge(style="dotted") - PV("500Mi volume\n(dynamic)")
            #pod1 - Edge(style="dotted") - PVC("redis-data-redis-1\n(dynamic)") - Edge(style="dotted") - PV("500Mi volume\n(dynamic)")
            statefulset - Edge(style="dashed") - pods << configmap
            
            
# Prometheus
with Diagram("Provisionamento da ferramenta de monitoramento Prometheus", show=False, outformat="png", graph_attr=graph_attr, filename="prometheus_diagram"):
    with Cluster("RBAC"):
        role = ClusterRole("prometheus-clusterrole")
        rolebinding = ClusterRoleBinding("prometheus-clusterrolebinding")
        account = ServiceAccount("prometheus-serviceaccount")
        role << Edge(label="binds") << rolebinding >> Edge(label="binds") >> account

    with Cluster("Prometheus"):
        configmap = ConfigMap("prometheus-configmap\n(mounted as volume)")
        deployment = Deployment("prometheus-deployment")
        service = Service("prometheus-svc\n(ClusterIP)")

        service >> deployment
        with Cluster("ReplicaSet"):
            pods = [Pod("prometheus")]
            account - pods
            deployment - Edge(style="dashed") - pods
            pods << configmap

# RabbitMQ
with Diagram("Provisionamento do message broker RabbitMQ", show=False, outformat="png", graph_attr=graph_attr, filename="rabbitmq_diagram"):
    with Cluster("RabbitMQ"):
        plugin_config = ConfigMap("rabbitmq-plugin-configmap\n(mounted as volume)")
        configmap = ConfigMap("rabbitmq-configmap\n(mounted as volume)")
        pvc = PVC("rabbitmq-pvc")
        volume = PV("2Gi volume\n(dynamic)")
        deployment = Deployment("rabbitmq-deployment")
        service = Service("rabbitmq\n(ClusterIP)")

        service >> deployment
        deployment << pvc
        with Cluster("ReplicaSet"):
            pods = [Pod("rabbitmq")]
            deployment - Edge(style="dashed") - pods
            pods << plugin_config
            pods << configmap
            pods - Edge(style="dotted") - volume

# Grafana
with Diagram("Provisionamento do dashboard de monitoramento Grafana", show=False, outformat="png", graph_attr=graph_attr, filename="grafana_diagram"):
    with Cluster("Grafana"):
        datasources = ConfigMap("grafana-provisioning-datasources-configmap\n(mounted as volume)")
        dashboards_1 = ConfigMap("grafana-provisioning-dashboards-configmap\n(mounted as volume)")
        dashboards_2 = ConfigMap("grafana-dashboards-configmap\n(mounted as volume)")
        configmap = ConfigMap("grafana-configmap\n(mounted as volume)")
        pvc = PVC("grafana-pvc")
        volume = PV("1Gi volume\n(dynamic)")
        deployment = Deployment("grafana-deployment")
        service = Service("grafana\n(LoadBalancer)")

        service >> deployment
        deployment << pvc
        with Cluster("ReplicaSet"):
            pods = [Pod("grafana")]
            deployment - Edge(style="dashed") - pods
            pods << datasources
            pods << dashboards_1
            pods << dashboards_2
            pods << configmap
            pods - Edge(style="dotted") - volume

# Fluentd
with Diagram("Provisionamento do agregador de logs Fluentd", show=False, outformat="png", graph_attr=graph_attr, filename="fluentd_diagram"):
    node = Node("Node")
    with Cluster("RBAC"):
        serviceaccount = ServiceAccount("fluentd-serviceaccount")
        clusterrole = ClusterRole("fluentd-clusterrole")
        clusterrolebinding = ClusterRoleBinding("fluentd-clusterrolebinding")
        clusterrole << Edge(label="binds") << clusterrolebinding >> Edge(label="binds") >> serviceaccount

    with Cluster("Fluentd"):
        configmap = ConfigMap("fluentd-configmap")
        daemonset = DaemonSet("fluentd-daemonset")
        with Cluster("Node Replica"):
            pod = Pod("fluentd")
            node - Edge(style="dashed") - pod
            serviceaccount - pod
            daemonset - Edge(style="dashed") - pod
            pod << configmap

# Elasticsearch
with Diagram("Provisionamento da engine de busca e analytics Elasticsearch", show=False, outformat="png", graph_attr=graph_attr, filename="elasticsearch_diagram"):
    with Cluster("Elasticsearch"):
        service = Service("elasticsearch-service")
        statefulset = StatefulSet("elasticsearch-statefulset")
        service >> statefulset
        with Cluster("ReplicaSet"):
            pod = Pod("elasticsearch-statefulset-0")
            statefulset - Edge(style="dashed") - pod

# Kibana
with Diagram("Provisionamento da interface Kibana", show=False, outformat="png", graph_attr=graph_attr, filename="kibana_diagram"):
    with Cluster("Kibana"):
        configmap = ConfigMap("kibana-configmap")
        deployment = Deployment("kibana-deployment")
        service = Service("kibana-svc")
        service >> deployment
        with Cluster("ReplicaSet"):
            pods = [Pod("kibana")]
            deployment - Edge(style="dashed") - pods
            pods << configmap

# PgAdmin4
# Mongo Express
# Redis Commander


# Fluentd DaemonSet (página sobre logs)
with Diagram("Coleta de logs no cluster Kubernetes", show=False, outformat="png", graph_attr=graph_attr, filename="log_collect_diagram"):
    node1 = Node("Cluster Node #1")
    node2 = Node("Cluster Node #2")
    node3 = Node("Cluster Node #3")

    with Cluster("Fluentd DaemonSet"):
        fluentd1 = Fluentd("Node #1 Pod")
        fluentd2 = Fluentd("Node #2 Pod")
        fluentd3 = Fluentd("Node #3 Pod")
        node1 - Edge(style="dashed", label="read node logs") - fluentd1
        node2 - Edge(style="dashed", label="read node logs") - fluentd2
        node3 - Edge(style="dashed", label="read node logs") - fluentd3

    with Cluster("Elastic"):
        elastic = Elasticsearch("Elasticsearch")
        kibana = Kibana("Kibana")
        elastic - Edge(style="dashed", label="share info") - kibana
        
    [fluentd1, fluentd2, fluentd3] >> elastic
    kibana >> User("User")
