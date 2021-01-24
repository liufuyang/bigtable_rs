$(gcloud beta emulators bigtable env-init)

echo "To run examples with emulator, remember to set ENV param BIGTABLE_EMULATOR_HOST=$BIGTABLE_EMULATOR_HOST"

# https://cloud.google.com/bigtable/docs/quickstart-cbt
cbt -instance instance-1 -project emulator  createtable table-1
cbt -instance instance-1 -project emulator  ls
cbt -instance instance-1 -project emulator  createfamily table-1 cf1
cbt -instance instance-1 -project emulator  createfamily table-1 cf2
cbt -instance instance-1 -project emulator  ls table-1

cbt -instance instance-1 -project emulator  set table-1 key1 cf1:c1=value1
cbt -instance instance-1 -project emulator  set table-1 key2 cf1:c1=value2
cbt -instance instance-1 -project emulator  set table-1 key3 cf1:c1=value3
cbt -instance instance-1 -project emulator  set table-1 key4 cf1:c1=value4
cbt -instance instance-1 -project emulator  set table-1 key5 cf1:c1=value5
cbt -instance instance-1 -project emulator  set table-1 key6 cf1:c1=value6

# Set another version into key1
cbt -instance instance-1 -project emulator  set table-1 key1 cf1:c1=value1.v1
cbt -instance instance-1 -project emulator  set table-1 key1 cf1:c2=value1.c2

# Set another qualifier on key2
cbt -instance instance-1 -project emulator  set table-1 key2 cf1:c2=value2.c2

# Set another family on key3
cbt -instance instance-1 -project emulator  set table-1 key3 cf2:c1=value3.cf2


cbt -instance instance-1 -project emulator  read table-1