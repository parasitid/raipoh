ok. i want to write a small AI assistant tool which analyzes a whole git repository and generates a README.ai.md, 
something like the readme.ai.md at the end of this file.

This readme shall document the architecture of the project in the repo in order to feed coding ai assistants so they can
produce better change proposals. 

as this file cannot be generated at once:
the tool shall iterate incrementally starting with:
* optional inputs from user,
* the global readme and root directory files
* the docs
* then generate the whole directory structure by analysing which package/directory file.

each step should be small enough store its knowledge in a local sqlite db.
the knowledge doc should be produced by extracting info from the DB.

the loop is the following:

- knowledge file inexistent?
> gather global info from repo root files by asking llm. store result in sqlite
> generate first global version of knowledge.
> gather info from doc by asking llm, providing up to date knowledge, store info, regenerate knowledge file
> gather info from directory structure, descending the tree level by level, ask llm, provide incremental knowledge , up to the last levem
> generate architecture diagrams and so on.

in the end, the knowledge file can be rebuilt from scratch from the local db.
it should be properly formatted and comprehensible by any code ai assisant as a prompt file 
the tool shall be written in rust, with llm backends such as claude, openai, openrouter and so on.

USE the rust crate rig.

it should be available as a cli or a lib

anytime an analysis fail for whatever reason, the tool should be able to be relaunched, and restart at the last failed step.



README.ai.md example:

``` markdown
   # Archer Architecture Knowledge Base
   
   ## Overview
   Archer is a CCloud Endpoint Service enabling private connectivity between OpenStack networks. It allows injection of services into consumer networks via private IP addresses while enforcing quotas and security policies. The system supports two distinct service provider types:
   - **"cp"**: Cloud Provider services that use network injection backend
   - **"tenant"**: Tenant-managed services that use F5 BigIP backend
   
   ## Project Structure
   
   ```
   .
   ├── COPYRIGHT.txt
   ├── Dockerfile
   ├── LICENSE
   ├── Makefile
   ├── README.md
   ├── client
   │   ├── archer_client.go
   │   ├── endpoint
   │   │   ├── delete_endpoint_endpoint_id_parameters.go
   │   │   ├── delete_endpoint_endpoint_id_responses.go
   │   │   ├── endpoint_client.go
   │   │   ├── get_endpoint_endpoint_id_parameters.go
   │   │   ├── get_endpoint_endpoint_id_responses.go
   │   │   ├── get_endpoint_parameters.go
   │   │   ├── get_endpoint_responses.go
   │   │   ├── post_endpoint_parameters.go
   │   │   ├── post_endpoint_responses.go
   │   │   ├── put_endpoint_endpoint_id_parameters.go
   │   │   └── put_endpoint_endpoint_id_responses.go
   │   ├── quota
   │   │   ├── delete_quotas_project_id_parameters.go
   │   │   ├── delete_quotas_project_id_responses.go
   │   │   ├── get_quotas_defaults_parameters.go
   │   │   ├── get_quotas_defaults_responses.go
   │   │   ├── get_quotas_parameters.go
   │   │   ├── get_quotas_project_id_parameters.go
   │   │   ├── get_quotas_project_id_responses.go
   │   │   ├── get_quotas_responses.go
   │   │   ├── put_quotas_project_id_parameters.go
   │   │   ├── put_quotas_project_id_responses.go
   │   │   └── quota_client.go
   │   ├── rbac
   │   │   ├── delete_rbac_policies_rbac_policy_id_parameters.go
   │   │   ├── delete_rbac_policies_rbac_policy_id_responses.go
   │   │   ├── get_rbac_policies_parameters.go
   │   │   ├── get_rbac_policies_rbac_policy_id_parameters.go
   │   │   ├── get_rbac_policies_rbac_policy_id_responses.go
   │   │   ├── get_rbac_policies_responses.go
   │   │   ├── post_rbac_policies_parameters.go
   │   │   ├── post_rbac_policies_responses.go
   │   │   ├── put_rbac_policies_rbac_policy_id_parameters.go
   │   │   ├── put_rbac_policies_rbac_policy_id_responses.go
   │   │   └── rbac_client.go
   │   ├── service
   │   │   ├── delete_service_service_id_parameters.go
   │   │   ├── delete_service_service_id_responses.go
   │   │   ├── get_service_parameters.go
   │   │   ├── get_service_responses.go
   │   │   ├── get_service_service_id_endpoints_parameters.go
   │   │   ├── get_service_service_id_endpoints_responses.go
   │   │   ├── get_service_service_id_parameters.go
   │   │   ├── get_service_service_id_responses.go
   │   │   ├── post_service_parameters.go
   │   │   ├── post_service_responses.go
   │   │   ├── put_service_service_id_accept_endpoints_parameters.go
   │   │   ├── put_service_service_id_accept_endpoints_responses.go
   │   │   ├── put_service_service_id_parameters.go
   │   │   ├── put_service_service_id_reject_endpoints_parameters.go
   │   │   ├── put_service_service_id_reject_endpoints_responses.go
   │   │   ├── put_service_service_id_responses.go
   │   │   └── service_client.go
   │   └── version
   │       ├── get_parameters.go
   │       ├── get_responses.go
   │       └── version_client.go
   ├── cmd
   │   ├── archer-f5-agent
   │   │   └── main.go
   │   ├── archer-migrate
   │   │   └── main.go
   │   ├── archer-ni-agent
   │   │   └── main.go
   │   ├── archer-server
   │   │   └── main.go
   │   └── archerctl
   │       └── main.go
   ├── etc
   │   ├── archer.example.ini
   │   └── policy.json
   ├── go.mod
   ├── go.sum
   ├── internal
   │   ├── agent
   │   │   ├── common.go
   │   │   ├── common_test.go
   │   │   ├── f5
   │   │   │   ├── agent.go
   │   │   │   ├── as3
   │   │   │   │   ├── as3types.go
   │   │   │   │   ├── bigip.go
   │   │   │   │   ├── bigip_interface.go
   │   │   │   │   ├── bigip_test.go
   │   │   │   │   ├── extended_endpoint.go
   │   │   │   │   ├── extended_service.go
   │   │   │   │   ├── mock_BigIPIface.go
   │   │   │   │   ├── proxy_protocol_2.go
   │   │   │   │   └── routedomain.go
   │   │   │   ├── check.go
   │   │   │   ├── check_test.go
   │   │   │   ├── cleanup.go
   │   │   │   ├── cleanup_test.go
   │   │   │   ├── endpoints.go
   │   │   │   ├── endpoints_test.go
   │   │   │   ├── l2.go
   │   │   │   ├── l2_test.go
   │   │   │   ├── services.go
   │   │   │   └── services_test.go
   │   │   ├── logger.go
   │   │   ├── ni
   │   │   │   ├── agent.go
   │   │   │   ├── haproxy.go
   │   │   │   ├── netlink.go
   │   │   │   ├── openstack.go
   │   │   │   └── service.go
   │   │   └── prometheus.go
   │   ├── auth
   │   │   └── keystone.go
   │   ├── client
   │   │   ├── client.go
   │   │   ├── endpoint.go
   │   │   ├── quota.go
   │   │   ├── rbac.go
   │   │   ├── service.go
   │   │   ├── table_writer.go
   │   │   └── version.go
   │   ├── config
   │   │   └── config.go
   │   ├── controller
   │   │   ├── controller.go
   │   │   ├── endpoint.go
   │   │   ├── endpoint_test.go
   │   │   ├── notify.go
   │   │   ├── quota.go
   │   │   ├── quota_test.go
   │   │   ├── rbac.go
   │   │   ├── rbac_test.go
   │   │   ├── service.go
   │   │   ├── service_test.go
   │   │   ├── set_defaults.go
   │   │   ├── suite_test.go
   │   │   ├── version.go
   │   │   └── version_test.go
   │   ├── db
   │   │   ├── logger.go
   │   │   ├── migrations
   │   │   │   └── migration.go
   │   │   ├── pagination_helper.go
   │   │   ├── pagination_test.go
   │   │   ├── pgx_interface.go
   │   │   ├── query_generator.go
   │   │   ├── quota.go
   │   │   └── quota_test.go
   │   ├── errors
   │   │   └── errors.go
   │   ├── middlewares
   │   │   ├── audit.go
   │   │   └── healthcheck.go
   │   ├── neutron
   │   │   ├── multiport.go
   │   │   ├── neutron.go
   │   │   ├── neutron_test.go
   │   │   ├── portsbinding.go
   │   │   └── portsbinding_test.go
   │   ├── policy
   │   │   ├── goslo.go
   │   │   ├── noop.go
   │   │   └── policy.go
   │   └── unique.go
   ├── models
   │   ├── endpoint.go
   │   ├── endpoint_consumer.go
   │   ├── endpoint_consumer_list.go
   │   ├── endpoint_status.go
   │   ├── error.go
   │   ├── link.go
   │   ├── project.go
   │   ├── quota.go
   │   ├── quota_usage.go
   │   ├── rbacpolicy.go
   │   ├── rbacpolicycommon.go
   │   ├── service.go
   │   ├── service_updatable.go
   │   ├── timestamp.go
   │   └── version.go
   ├── restapi
   │   ├── configure_archer.go
   │   ├── doc.go
   │   ├── embedded_spec.go
   │   ├── operations
   │   │   ├── archer_api.go
   │   │   ├── endpoint
   │   │   │   ├── delete_endpoint_endpoint_id.go
   │   │   │   ├── delete_endpoint_endpoint_id_parameters.go
   │   │   │   ├── delete_endpoint_endpoint_id_responses.go
   │   │   │   ├── delete_endpoint_endpoint_id_urlbuilder.go
   │   │   │   ├── get_endpoint.go
   │   │   │   ├── get_endpoint_endpoint_id.go
   │   │   │   ├── get_endpoint_endpoint_id_parameters.go
   │   │   │   ├── get_endpoint_endpoint_id_responses.go
   │   │   │   ├── get_endpoint_endpoint_id_urlbuilder.go
   │   │   │   ├── get_endpoint_parameters.go
   │   │   │   ├── get_endpoint_responses.go
   │   │   │   ├── get_endpoint_urlbuilder.go
   │   │   │   ├── post_endpoint.go
   │   │   │   ├── post_endpoint_parameters.go
   │   │   │   ├── post_endpoint_responses.go
   │   │   │   ├── post_endpoint_urlbuilder.go
   │   │   │   ├── put_endpoint_endpoint_id.go
   │   │   │   ├── put_endpoint_endpoint_id_parameters.go
   │   │   │   ├── put_endpoint_endpoint_id_responses.go
   │   │   │   └── put_endpoint_endpoint_id_urlbuilder.go
   │   │   ├── quota
   │   │   │   ├── delete_quotas_project_id.go
   │   │   │   ├── delete_quotas_project_id_parameters.go
   │   │   │   ├── delete_quotas_project_id_responses.go
   │   │   │   ├── delete_quotas_project_id_urlbuilder.go
   │   │   │   ├── get_quotas.go
   │   │   │   ├── get_quotas_defaults.go
   │   │   │   ├── get_quotas_defaults_parameters.go
   │   │   │   ├── get_quotas_defaults_responses.go
   │   │   │   ├── get_quotas_defaults_urlbuilder.go
   │   │   │   ├── get_quotas_parameters.go
   │   │   │   ├── get_quotas_project_id.go
   │   │   │   ├── get_quotas_project_id_parameters.go
   │   │   │   ├── get_quotas_project_id_responses.go
   │   │   │   ├── get_quotas_project_id_urlbuilder.go
   │   │   │   ├── get_quotas_responses.go
   │   │   │   ├── get_quotas_urlbuilder.go
   │   │   │   ├── put_quotas_project_id.go
   │   │   │   ├── put_quotas_project_id_parameters.go
   │   │   │   ├── put_quotas_project_id_responses.go
   │   │   │   └── put_quotas_project_id_urlbuilder.go
   │   │   ├── rbac
   │   │   │   ├── delete_rbac_policies_rbac_policy_id.go
   │   │   │   ├── delete_rbac_policies_rbac_policy_id_parameters.go
   │   │   │   ├── delete_rbac_policies_rbac_policy_id_responses.go
   │   │   │   ├── delete_rbac_policies_rbac_policy_id_urlbuilder.go
   │   │   │   ├── get_rbac_policies.go
   │   │   │   ├── get_rbac_policies_parameters.go
   │   │   │   ├── get_rbac_policies_rbac_policy_id.go
   │   │   │   ├── get_rbac_policies_rbac_policy_id_parameters.go
   │   │   │   ├── get_rbac_policies_rbac_policy_id_responses.go
   │   │   │   ├── get_rbac_policies_rbac_policy_id_urlbuilder.go
   │   │   │   ├── get_rbac_policies_responses.go
   │   │   │   ├── get_rbac_policies_urlbuilder.go
   │   │   │   ├── post_rbac_policies.go
   │   │   │   ├── post_rbac_policies_parameters.go
   │   │   │   ├── post_rbac_policies_responses.go
   │   │   │   ├── post_rbac_policies_urlbuilder.go
   │   │   │   ├── put_rbac_policies_rbac_policy_id.go
   │   │   │   ├── put_rbac_policies_rbac_policy_id_parameters.go
   │   │   │   ├── put_rbac_policies_rbac_policy_id_responses.go
   │   │   │   └── put_rbac_policies_rbac_policy_id_urlbuilder.go
   │   │   ├── service
   │   │   │   ├── delete_service_service_id.go
   │   │   │   ├── delete_service_service_id_parameters.go
   │   │   │   ├── delete_service_service_id_responses.go
   │   │   │   ├── delete_service_service_id_urlbuilder.go
   │   │   │   ├── get_service.go
   │   │   │   ├── get_service_parameters.go
   │   │   │   ├── get_service_responses.go
   │   │   │   ├── get_service_service_id.go
   │   │   │   ├── get_service_service_id_endpoints.go
   │   │   │   ├── get_service_service_id_endpoints_parameters.go
   │   │   │   ├── get_service_service_id_endpoints_responses.go
   │   │   │   ├── get_service_service_id_endpoints_urlbuilder.go
   │   │   │   ├── get_service_service_id_parameters.go
   │   │   │   ├── get_service_service_id_responses.go
   │   │   │   ├── get_service_service_id_urlbuilder.go
   │   │   │   ├── get_service_urlbuilder.go
   │   │   │   ├── post_service.go
   │   │   │   ├── post_service_parameters.go
   │   │   │   ├── post_service_responses.go
   │   │   │   ├── post_service_urlbuilder.go
   │   │   │   ├── put_service_service_id.go
   │   │   │   ├── put_service_service_id_accept_endpoints.go
   │   │   │   ├── put_service_service_id_accept_endpoints_parameters.go
   │   │   │   ├── put_service_service_id_accept_endpoints_responses.go
   │   │   │   ├── put_service_service_id_accept_endpoints_urlbuilder.go
   │   │   │   ├── put_service_service_id_parameters.go
   │   │   │   ├── put_service_service_id_reject_endpoints.go
   │   │   │   ├── put_service_service_id_reject_endpoints_parameters.go
   │   │   │   ├── put_service_service_id_reject_endpoints_responses.go
   │   │   │   ├── put_service_service_id_reject_endpoints_urlbuilder.go
   │   │   │   ├── put_service_service_id_responses.go
   │   │   │   └── put_service_service_id_urlbuilder.go
   │   │   └── version
   │   │       ├── get.go
   │   │       ├── get_parameters.go
   │   │       ├── get_responses.go
   │   │       └── get_urlbuilder.go
   │   └── server.go
   └── swagger.yaml
   ```
   
   **Policy File Details**:
   - Located in `etc/policy.json`  
   - Defines RBAC rules in OpenStack-style format
   - Used by `goslo` policy engine
   - Contains rules for:
     - Cloud admin privileges
     - Service/endpoint quotas
     - RBAC policy management
   
   ## Component Architecture
   ### 1. CLI Client (`archerctl`)
   - **Authentication**:
     - Supports OpenStack RC environment variables
     - Direct parameters for:
       - `--os-username`
       - `--os-password`
       - `--os-project-name`
       - `--os-auth-url`
   - **Capabilities**:
     - Resource management (services, endpoints, quotas)
     - Output customization (format, columns, sorting)
     - Debug mode for verbose logging
   - **Service Creation**:
     - Requires specifying provider type (`--provider` flag)
     - Example: `archerctl service create --provider cp` or `archerctl service create --provider tenant`
   
   ### 2. API Server
   - **Core Responsibilities**:
     - RESTful API endpoint management
     - Quota enforcement (deployment defaults + project overrides)
     - Policy validation (OpenStack-style `policy.json`)
     - Database persistence (PostgreSQL)
     - Audit logging (CADF-compliant)
     - Provider type routing:
       - "cp" services → Network Injection Agent
       - "tenant" services → F5 BigIP
   
   - **Middleware Stack**:
     - Keystone token validation
     - Request context management
     - Rate limiting
     - CORS handling
     - Audit tracing
   
   - **API Features**:
     - Pagination (`limit`, `marker`, `page_reverse`)
     - Multi-key sorting (`?sort=key1,-key2`)
     - Tag filtering (`tags`, `tags-any`, `not-tags`, `not-tags-any`)
   
   ### 3. Network Injection Agent
   - **Provider Type Scope**:
     - Only watches services with provider type "cp"
     - Ignores "tenant" services
   - **Supported Implementations**:
     - OpenVSwitch agent
     - Linuxbridge agent
   - **Functions**:
     - Inject services into consumer networks
     - Manage endpoint IP allocation
     - Maintain network connectivity state
     - Synchronize with API server
     - Handle VLAN restrictions and network flows
   
   ### 4. F5 BigIP Agent
   - **Provider Type Scope**:
     - Only watches services with provider type "tenant"
     - Ignores "cp" services
   - **Integration**:
     - AS3 configuration via `as3types.Service`
     - Uses `ProfileL4` and other AS3-specific configurations
   - **Features**:
     - VLAN restrictions
     - iRules
     - Mirroring
     - Persistence methods
     - Pool configuration
   
   ### 5. Backend Systems
   1. **F5 BigIP**
      - AS3 configuration via `as3types.Service`
      - Used exclusively for "tenant" provider type
      - Features:
        - VLAN restrictions
        - iRules
        - Mirroring
        - Persistence methods
        - Pool configuration
   
   2. **Network Injection Backend**
      - Used exclusively for "cp" provider type
      - Implements service injection through:
        - OpenVSwitch flows
        - Linuxbridge VLAN tagging
      - Integrates with Neutron for network context
   
   3. **PostgreSQL**
      - Persistent storage for:
        - Service configurations (including provider type)
        - Endpoint records
        - Quota settings
        - RBAC policies
   
   4. **OpenStack Integration**
      - Keystone: Authentication and token validation
      - Neutron: Network topology awareness
      - Nova: Instance metadata (indirect through Neutron)
   
   ## Component Interactions
   ### Data Flow
   1. **CLI → API Server**
      - REST API calls with Keystone token authentication
      - Provider type determines backend routing
      - Example: `archerctl service create --provider cp` → POST /v1/services → Network Injection Agent
      - Example: `archerctl service create --provider tenant` → POST /v1/services → F5 BigIP
   
   2. **API Server → Database**
      - PostgreSQL operations for:
        - Resource persistence (including provider type)
        - Quota tracking
        - Policy enforcement
   
   3. **API Server → Agents**
      - Provider type-based routing:
        - "cp" → Network Injection Agent
        - "tenant" → F5 BigIP
   
   4. **Network Injection Agent → Backend Systems**
      - OVS/Linuxbridge operations for "cp" services
      - Direct network configuration in consumer networks
   
   5. **F5 Agent → Backend Systems**
      - AS3 configuration updates for "tenant" services
      - F5-specific network policies
   
   ## Deployment Architecture
   ```
   +---------------------+     +-------------------+
   |     archerctl CLI   |<--->|   Archer API      |
   +---------------------+     |   Server          |
                               |-------------------|
                               | Middleware Stack  |
                               | - Keystone Auth   |
                               | - Policy Engine   |
                               | - Audit Logging   |
                               +-------------------+
                                         |
                                         v
                      +------------------------------------+
                      |     PostgreSQL Database            |
                      | - Services (provider type aware)   |
                      | - Endpoints                      |
                      | - Quotas                         |
                      | - RBAC policies                  |
                      +------------------------------------+
                                         |
           +---------------------------+---------------------------+
           |                                                           |
           v                                                           v
   +---------------------------+                         +---------------------------+
   | Network Injection Agent    |                         | F5 BigIP Agent              |
   | - Watches provider=cp      |                         | - Watches provider=tenant   |
   | - OVS/Linuxbridge backend  |                         | - AS3 configuration        |
   +---------------------------+                         +---------------------------+
           |                                                           |
           v                                                           v
   +---------------------------+                         +---------------------------+
   | OpenStack Neutron         |                         | F5 BigIP AS3              |
   | - Network context         |                         | - Service configuration     |
   | - IP allocation           |                         | - Network policies          |
   +---------------------------+                         +---------------------------+
   ```
   
   ## Communication Patterns
   1. **CLI ↔ API Server**
      - RESTful JSON API
      - Token authentication via `X-Auth-Token`
      - Provider type specified in request body or CLI flags
   
   2. **API Server ↔ Database**
      - Direct PostgreSQL connections
      - ORM-based data access
      - Provider type stored in service records
   
   3. **API Server ↔ Agents**
      - Provider type determines communication path:
        - "cp" → Network Injection Agent
        - "tenant" → F5 BigIP
      - Each agent only processes services of its designated provider type
   
   4. **Agents ↔ Backend Systems**
      - Network Injection:
        - OVS flow programming
        - Linuxbridge VLAN tagging
        - Neutron API for network context
      - F5 BigIP:
        - AS3 configuration API
        - F5-specific network policies
   
   ## Operational Workflow
   1. **Service Creation**
      - CLI sends service creation request with provider type
      - API server validates token and policy
      - Checks service quotas
      - Persists service in PostgreSQL with provider type
      - Routes to appropriate backend:
        - "cp" → Network Injection Agent
        - "tenant" → F5 BigIP
   
   2. **Endpoint Injection**
      - CLI requests endpoint creation
      - API server validates project scope
      - Allocates IP address
      - Tracks endpoint quota usage
      - Instructs appropriate agent based on service provider type
   
   3. **Status Monitoring**
      - Agents report endpoint status for their provider type
      - API server updates PostgreSQL
      - Prometheus scrapes metrics from API server
      - Sentry captures critical errors
   
   ## Key Implementation Details
   1. **Provider Type Routing**
      - Service records include provider type metadata
      - Agents filter services by provider type:
        - Network Injection Agent → provider=cp
        - F5 Agent → provider=tenant
      - API server routes requests based on provider type
   
   2. **Quota Enforcement**
      - Unified quota system for both provider types
      - Separate counters for endpoints and services
      - Enforcement happens at API layer before backend processing
   
   3. **Security Model**
      - Token validation through Keystone middleware
      - Policy enforcement via `policy.json`
      - Unified 403 response for both:
        - Policy violations
        - Quota overflows
   
   4. **Network Injection**
      - "cp" provider type implementation:
        - OpenVSwitch agent for flow-based injection
        - Linuxbridge agent for VLAN-based injection
      - Provider type determines backend capabilities
   
   5. **F5 Integration**
      - "tenant" provider type implementation
      - AS3 configuration through:
        - Profiles
        - iRules
        - Pool management
        - Persistence profiles
        - Virtual server configuration
   
   ## Operational Requirements
   1. **Core Services**
      - PostgreSQL for persistent storage
      - Keystone for authentication
      - Neutron for network context
   
   2. **Backend Requirements**
      - For "cp" services:
        - Network Injection Agent
        - OVS/Linuxbridge infrastructure
      - For "tenant" services:
        - F5 BigIP infrastructure
        - AS3 service configuration
   
   3. **Monitoring**
      - Prometheus metrics exporter
      - Sentry error tracking
   
   4. **High Availability**
      - Stateless API servers
      - Shared PostgreSQL database
      - Independent backend agents
   
   ## Response Codes
   ```markdown
   | Code | Description                                                                 |
   |------|-----------------------------------------------------------------------------|
   | 400  | Validation error                                                            |
   | 401  | Unauthorized                                                                |
   | 403  | Policy denied or quota exceeded                                              |
   | 404  | Not Found (resource or network)                                              |
   | 409  | Conflict (duplicate resource)                                                 |
   | 429  | Rate limit exceeded                                                         |
   | 500  | Internal server error                                                       |
   ```
   
   ## Middleware
   - Custom context handling via `middleware.Context`
   - Audit middleware tracks:
     - User identity
     - Request metadata
     - Policy enforcement
   - Security principal extraction from tokens
   - Rate limiting and CORS support
```
