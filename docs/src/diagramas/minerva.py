from diagrams import Cluster, Diagram, Edge

from diagrams.k8s.clusterconfig import HPA
from diagrams.k8s.compute import Deployment, Pod, ReplicaSet, StatefulSet, Job
from diagrams.k8s.network import Ingress, Service
from diagrams.k8s.storage import PV, PVC
from diagrams.k8s.podconfig import ConfigMap, Secret
from diagrams.k8s.rbac import ClusterRole, ClusterRoleBinding, ServiceAccount

from diagrams.onprem.queue import RabbitMQ
from diagrams.onprem.compute import Server
from diagrams.onprem.network import Traefik
from diagrams.onprem.database import PostgreSQL, MongoDB
from diagrams.onprem.inmemory import Redis
from diagrams.onprem.monitoring import Grafana, Prometheus

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
        grafana = Grafana("grafana")

    # Ingress
    ingress >> Edge(color="lightblue") >> rest
    ingress >> Edge(color="lightblue") >> grafana

    # Services dispatch
    rest >> Edge(color="black") << users
    rest >> Edge(color="black") << session
    dispatch >> Edge(color="black") >> session

    # Messaging
    dispatch >> Edge(color="darkorange") << rabbitmq
    session >> Edge(color="darkorange", style="dashed") >> rabbitmq

    # Cache
    session >> Edge(color="brown", label="Session cache") << redis

    # Databases
    session >> Edge(color="darkgreen", label="Session data") << mongo
    users >> Edge(color="blue") << postgres
    session >> Edge(color="blue") << postgres

    # Metrics
    prometheus << Edge(color="brown") << rabbitmq
    grafana >> Edge(color="darkorange", style="dashed") >> prometheus

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
        pvc = PVC("redis-pvc")
        volume = PV("500Mi volume\n(dynamic)")
        statefulset = StatefulSet("redis")
        service = Service("redis-svc\n(ClusterIP)")
        hpa = HPA("redis-hpa")

        service >> statefulset
        hpa >> statefulset
        statefulset << pvc
        with Cluster("ReplicaSet"):
            pods = [Pod("redis-0\n(master)"), Pod("redis-1")]
            statefulset - Edge(style="dashed") - pods
            pods << configmap
            pods - Edge(style="dotted") - volume
            
            
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

