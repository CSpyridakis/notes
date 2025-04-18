# RPM (Red Hat Package Manager)
RPM based Distros

| **Feature**             | **Fedora**                            | **CentOS Stream**                        | **Red Hat Enterprise Linux (RHEL)**                                |
|-------------------------|---------------------------------------|------------------------------------------|-----------------------------------------|
| **Purpose/Category**    | Cutting-edge, community-driven       | Rolling-release, pre-RHEL                | Enterprise-focused                      |
| **Release Cycle**       | Rapid, 6-month releases              | Rolling (frequent updates)               | Long-term support (10 years)            |
| **Target Audience**     | Developers, tech enthusiasts         | Enterprises, server environments         | Enterprises, production environments    |
| **Stability**           | Less stable, newer features          | Less stable than CentOS Linux            | Very stable                             |
| **Updates and Patches** | Frequent updates, newer software     | Updates before RHEL                      | Enterprise-level, tested patches        |
| **Support**             | Community support                    | Community support                        | Paid support from Red Hat               |
| **Paid/Free**           | Free                                  | Free                                     | Paid                                    |
| **Key Use Case**        | Developers, early adopters, desktop use | RHEL-compatible server use, pre-release RHEL | Production, enterprise, server use      |
| **Mandatory Things to Know** | Focuses on innovation, not long-term stability | CentOS Stream is a **rolling release** that precedes RHEL | Requires paid subscription for support, ideal for large-scale production environments |


## Package manager
Both `yum` and `dnf` are package management tools used on RPM-based Linux distributions (like Fedora, CentOS, and Red Hat), but they differ in design, performance, and features. 

- `yum` (Yellowdog Updater Modified):
    - Developed as a front-end to RPM, yum has been the traditional package manager for many RPM-based systems. It simplified dependency management compared to the raw RPM tool.
    - Remains available on some distributions for legacy support.

- `dnf` (Dandified Yum):
    - Introduced as the next-generation version of yum, dnf was created to address limitations in yum. It’s now the default package manager in many newer Fedora and Red Hat systems.
    - Aims to be largely compatible with yum command syntax to ease the transition, but there may be subtle differences in behavior or available options.