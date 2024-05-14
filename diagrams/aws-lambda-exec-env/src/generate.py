from diagrams import Cluster, Diagram

from diagrams.aws.compute import Lambda
from diagrams.oci.connectivity import Backbone
from diagrams.oci.storage import BlockStorage
from diagrams.oci.network import Vcn

cluster_style = { "fontsize": "28" }

with Diagram("AWS Lambda Execution Environment", show=False, filename="assets/aws-lambda-exec-env", outformat="pdf"):
    with Cluster("KVM on EC2", graph_attr=cluster_style):
        with Cluster("KVM Host Kernel", graph_attr=cluster_style):
            host_blk = BlockStorage("host-block")
            host_net = Backbone("host-net")
            host_vsock = Vcn("host-vsock")

        for tenant in range(2):
            with Cluster(f"Firecracker MicroVM (Tenant {tenant})", graph_attr=cluster_style):
                with Cluster("MicroVM Kernel", graph_attr=cluster_style):
                    virtio_blk = BlockStorage("virtio-block")
                    virtio_net = Backbone("virtio-net")
                    virtio_vsock = Vcn("virtio-vsock")

                    if tenant == 0:
                        virtio_blk >> host_blk
                        virtio_vsock >> host_vsock
                    if tenant == 1:
                        virtio_net >> host_net

                for fn_num in range(2):
                    with Cluster(f"Lambda Sandbox (Function {fn_num})", graph_attr=cluster_style):
                        with Cluster("Function Execution Environment", graph_attr=cluster_style):
                            fn = Lambda(f"Function {fn_num}")

                            if fn_num == 0:
                                fn >> virtio_blk
                            if fn_num == 1:
                                fn >> virtio_net
                                fn >> virtio_vsock
