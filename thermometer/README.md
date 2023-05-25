# thermometer

MQTT メッセージを発する温度センサのシミュレータ

## 使い方

Docker がインストールされている必要がある．

接続先の MQTT ブローカが `<host>:<port>` であるとき，
トピック `<topic>` に Publish するには次のコマンドを実行する．
```sh
docker run --rm --init thermometer <host> -p <port> -t <topic>
```

なお，`<port>` が `1883` である場合は `-p` オプションを省略できる．

全てのオプションは
```sh
docker run --rm --init thermometer -h
```
で確認できる．
