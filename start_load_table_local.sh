$(gcloud beta emulators bigtable env-init)

echo "To run examples with emulator, remember to set ENV param BIGTABLE_EMULATOR_HOST=$BIGTABLE_EMULATOR_HOST"

# https://cloud.google.com/bigtable/docs/quickstart-cbt
cbt -instance test1 -project emulator  createtable my-table
cbt -instance test1 -project emulator  ls
cbt -instance test1 -project emulator  createfamily my-table cf1
cbt -instance test1 -project emulator  ls my-table
cbt -instance test1 -project emulator  set my-table key1 cf1:c1=test-value-1
cbt -instance test1 -project emulator  set my-table key2 cf1:c1=test-value-2
cbt -instance test1 -project emulator  set my-table key3 cf1:c1=test-value-3
cbt -instance test1 -project emulator  set my-table key4 cf1:c1=test-value-4
cbt -instance test1 -project emulator  set my-table key5 cf1:c1=test-value-5
cbt -instance test1 -project emulator  set my-table key6 cf1:c1=test-value-6
cbt -instance test1 -project emulator  read my-table