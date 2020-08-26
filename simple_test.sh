#!/bin/bash

wait_for_input() {
    while [ true ] ; do read -t 3 -n 1
        if [ $? = 0 ] ; then
            echo -e "\n"
            break;
        fi
    done
}


echo -e "POST: https://localhost:8000/api/posts/"
id=$(curl -s -k -d '{"name":"¡no see!", "author":"The script"}' \
        -X POST https://localhost:8000/api/posts -H "Content-Type: application/json" | jq -r .id )
echo $id
wait_for_input

echo -e "GET: https://localhost:8000/api/posts/$id"
curl -k -i https://localhost:8000/api/posts/$id
wait_for_input

echo -e "PUT: https://localhost:8000/api/posts/$id"
curl -k -i -d '{ "name":"updated!" }' \
    -X PUT https://localhost:8000/api/posts/$id -H "Content-Type: application/json"
wait_for_input

echo -e "DELETE: https://localhost:8000/api/posts/$id"
curl -k -i \
    -X DELETE https://localhost:8000/api/posts/$id -H "Content-Type: application/json"
wait_for_input

echo -e "GET(404): https://localhost:8000/api/posts/$id"
curl -k -i https://localhost:8000/api/posts/$id
wait_for_input

echo -e "GET: https://localhost:8000/api/posts?name=smooth"
curl -k -i https://localhost:8000/api/posts?name=smooth
wait_for_input



