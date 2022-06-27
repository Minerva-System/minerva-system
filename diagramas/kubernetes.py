from diagrams import Cluster, Diagram
from diagrams.k8s.clusterconfig import HPA
from diagrams.k8s.compute import Deployment, Pod, ReplicaSet, StatefulSet, Job
from diagrams.k8s.network import Ingress, Service
from diagrams.k8s.storage import PV, PVC
from diagrams.k8s.podconfig import ConfigMap, Secret

graph_attr = {
    "bgcolor": "transparent"
}

with Diagram("Minerva System", show=False, outformat="png", graph_attr=graph_attr, filename="kubernetes"):
    #postgres_data = PV("postgresql_pv")
    #mongodb_data  = PV("mongodb_pv")
    #redis_data    = PV("redis-data")
    
    with Cluster("Ingress"):
        frontend_ing = Ingress("minerva-system.io/")
        rest_ing = Ingress("minerva-system.io/api")
    
    with Cluster("LoadBalancer"):
        frontend_svc = frontend_ing >> Service("frontend-svc")
        rest_svc = rest_ing >> Service("rest-svc")

    with Cluster("ClusterIP"):
        #ports_cm = ConfigMap("ports-configmap")
        #servers_cm = ConfigMap("servers-configmap")
        
        with Cluster("Front-End"):
            frontend_dp = frontend_svc >> Deployment("frontend-deployment")
            with Cluster(""):
                frontend_pod1 = Pod("frontend")
                frontend_pod2 = Pod("frontend")
            [frontend_pod1, frontend_pod2] - ConfigMap("frontend-configmap")
            frontend_dp >> [frontend_pod1, frontend_pod2]
            frontend_dp << HPA("frontend-hpa")

        with Cluster("RUNONCE"):
            runonce_job = Job("runonce-job")
            runonce_job - ConfigMap("runonce-configmap")
            runonce_job - Secret("runonce-secret")
            
        with Cluster("REST"):
            rest_dp = rest_svc >> Deployment("rest-deployment")
            with Cluster(""):
                rest_pod1 = Pod("rest")
                rest_pod2 = Pod("rest")
            [rest_pod1, rest_pod2] - ConfigMap("rest-configmap")
            [rest_pod1, rest_pod2] - ConfigMap("ports-configmap")
            [rest_pod1, rest_pod2] - ConfigMap("servers-configmap")
            rest_dp >> [rest_pod1, rest_pod2]
            rest_dp << HPA("rest-hpa")

        with Cluster("MongoDB"):
            mongodb_svc = Service("mongodb-svc")
            mongodb_dp = mongodb_svc >> Deployment("mongodb-deployment")
            mongodb_pod = mongodb_dp >> Pod("mongodb")
            mongodb_pvc = mongodb_dp - PVC("mongodb-pvc")
            mongodb_pod - ConfigMap("mongodb-configmap")
            mongodb_pvc << PV("mongodb_pv")

        with Cluster("PostgreSQL"):
            postgresql_svc = Service("postgresql-svc")  
            postgresql_dp = postgresql_svc >> Deployment("postgresql-deployment")
            postgresql_pod = postgresql_dp >> Pod("postgresql")
            postgresql_pvc = postgresql_dp - PVC("postgresql-pvc")
            postgresql_pod - ConfigMap("postgresql-configmap")
            postgresql_pvc << PV("postgresql_pv")
    
        with Cluster("USER"):
            user_svc = Service("user-svc")
            user_dp = user_svc >> Deployment("user-deployment")
            with Cluster(""):
                user_pod1 = Pod("user")
                user_pod2 = Pod("user")
            user_dp >> [user_pod1, user_pod2]
            [user_pod1, user_pod2] - ConfigMap("ports-configmap")
            [user_pod1, user_pod2] - ConfigMap("servers-configmap")
            user_dp << HPA("user-hpa")

        with Cluster("SESSION"):
            session_svc = Service("session-svc")
            session_dp = session_svc >> Deployment("session-deployment")
            with Cluster(""):
                session_pod1 = Pod("session")
                session_pod2 = Pod("session")
            session_dp >> [session_pod1, session_pod2]
            [session_pod1, session_pod2] - ConfigMap("ports-configmap")
            [session_pod1, session_pod2] - ConfigMap("servers-configmap")
            session_dp << HPA("session-hpa")

        with Cluster("Redis"):
            redis_svc = Service("redis-svc")
            redis_sts = redis_svc >> StatefulSet("redis")
            with Cluster(""):
                redis_pod0 = Pod("redis-0")
                redis_pod1 = Pod("redis-1")
                redis_pod2 = Pod("redis-2")
            redis_sts >> [redis_pod0, redis_pod1, redis_pod2]
            redis_sts << HPA("redis-hpa")
            redis_pvc = redis_sts - PVC("redis-pvc")
            redis_config_pv = redis_sts << PV("redis-config")
            redis_config_pv << ConfigMap("redis-configmap")
            redis_pvc << PV("redis-data")

        # Conexões externas a cada serviço
        frontend_dp >> rest_svc
        user_dp     >> postgresql_svc
        session_dp  >> [redis_svc, postgresql_svc, mongodb_svc]
        rest_dp     >> [user_svc, session_svc]
        runonce_job >> [postgresql_svc, mongodb_svc]
 
