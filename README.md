# 環境構築

Ubuntu 22.04.1 LTS

## 更新

```
apt update
apt dist-upgrade
apt autoremove
```

## 古いdockerバージョンの削除

```
apt-get remove docker docker-engine docker.io containerd runc
```

## dockerリポジトリの設定

### 必要なパッケージをインストールします。

```
apt-get install \
    ca-certificates \
    curl \
    gnupg \
    lsb-release
```

### Dockerの公式GPGキーを取得する

```
mkdir -p /etc/apt/keyrings
curl -fsSL https://download.docker.com/linux/ubuntu/gpg | sudo gpg --dearmor -o /etc/apt/keyrings/docker.gpg
```

### リポジトリの登録

```
echo \
  "deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/docker.gpg] https://download.docker.com/linux/ubuntu \
  $(lsb_release -cs) stable" | sudo tee /etc/apt/sources.list.d/docker.list > /dev/null
```

## Dockerのインストール

```
apt-get update
apt-get install docker-ce docker-ce-cli containerd.io docker-compose-plugin
```

## ibtablesの整合性の確保

```
update-alternatives --set iptables /usr/sbin/iptables-legacy
update-alternatives --set ip6tables /usr/sbin/ip6tables-legacy
```

## makeインストール

```
apt install make
```

## gitクローン

```
git clone git@github.com:cdq73700/rust.git rust
```

## .envコピー

```
cp backend/.env.local backend/.env
cp database/.env.local database/.env
```

## コンテナビルド

```
make build
```

## コンテナ作成

```
make up
```

## Cargoインストール

```
make ci
```

## VSCode実行

```
code .
```

## backend

http://localhost:4000/

## phpMyAdmin

http://localhost:8081/