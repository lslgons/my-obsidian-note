# Longhorn

[Longhorn | Quick Installation](https://longhorn.io/docs/1.8.0/deploy/install/)

노드별 Requirements

- iscsid
- nfs-utils

Ansible-playbook

```
# install_nfs_iscsi.yml
- name: Install NFS and iSCSI services
  hosts: primary-cluster
  remote_user: root
  become:true
  tasks:
    - name: Install nfs-utils and iscsid
      package:
        name:
          - nfs-utils
          - iscsi-initiator-utils# iscsid를 포함하는 패키지 (RHEL 계열)
          - cryptsetup
          - device-mapper
        state: present
    - name: Ensure iscsi_tcp module is loaded
      command: modprobe iscsi_tcp
      args:
        creates: /sys/module/iscsi_tcp
      register: iscsi_tcp_load
    - name: Persist iscsi_tcp module across reboots
      lineinfile:
        path: /etc/modules-load.d/iscsi_tcp.conf
        line: "iscsi_tcp"
        create: yes
        mode: "0644"
    - name: Ensure iscsid service is enabled and started
      systemd:
        name: iscsid
        enabled:true
        state: started
    - name: Ensure nfs-server service is enabled and started
      systemd:
        name: nfs-server
        enabled:true
        state: started

```

hosts.ini

```
[primary-cluster]
172.16.0.101
172.16.0.102
172.16.0.103
```

```
ansible-playbook -i ../hosts.ini install_nfs_iscsi.yml --ask-become-pass
```

# **Helm 차트**

[https://longhorn.io/docs/1.8.0/deploy/install/install-with-helm/](https://longhorn.io/docs/1.8.0/deploy/install/install-with-helm/)

```
helm repo add longhorn https://charts.longhorn.io
helm pull longhorn/longhorn --untar
helm install longhorn ./longhorn -n longhorn-system --create-namespace
```