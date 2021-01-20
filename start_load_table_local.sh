$(gcloud beta emulators bigtable env-init)

echo "To run examples with emulator, remember to set ENV param BIGTABLE_EMULATOR_HOST=$BIGTABLE_EMULATOR_HOST"

# https://cloud.google.com/bigtable/docs/quickstart-cbt
cbt -instance test1 -project emulator  createtable my-table
cbt -instance test1 -project emulator  ls
cbt -instance test1 -project emulator  createfamily my-table cf1
cbt -instance test1 -project emulator  ls my-table
cbt -instance test1 -project emulator  set my-table key1 cf1:c1=test-value-1
cbt -instance test1 -project emulator  read my-table