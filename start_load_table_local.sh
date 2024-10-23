$(gcloud beta emulators bigtable env-init)

echo "To run examples with emulator, remember to set ENV param BIGTABLE_EMULATOR_HOST=$BIGTABLE_EMULATOR_HOST"

# https://cloud.google.com/bigtable/docs/quickstart-cbt
cbt -instance instance-1 -project project-1  createtable table-1
cbt -instance instance-1 -project project-1  ls
cbt -instance instance-1 -project project-1  createfamily table-1 cf1
cbt -instance instance-1 -project project-1  createfamily table-1 cf2
cbt -instance instance-1 -project project-1  ls table-1

cbt -instance instance-1 -project project-1  set table-1 key1 cf1:c1=value1
cbt -instance instance-1 -project project-1  set table-1 key2 cf1:c1=value2
cbt -instance instance-1 -project project-1  set table-1 key3 cf1:c1=value3
cbt -instance instance-1 -project project-1  set table-1 key4 cf1:c1=value4
cbt -instance instance-1 -project project-1  set table-1 key5 cf1:c1=value5
cbt -instance instance-1 -project project-1  set table-1 key6 cf1:c1=value6

cbt -instance instance-1 -project project-1  set table-1 jey1 cf1:c1=jvalue1
cbt -instance instance-1 -project project-1  set table-1 jey2 cf1:c1=jvalue2
cbt -instance instance-1 -project project-1  set table-1 jey3 cf1:c1=jvalue3
cbt -instance instance-1 -project project-1  set table-1 jey4 cf1:c1=jvalue4
cbt -instance instance-1 -project project-1  set table-1 jey5 cf1:c1=jvalue5
cbt -instance instance-1 -project project-1  set table-1 jey6 cf1:c1=jvalue6

cbt -instance instance-1 -project project-1  set table-1 pey1 cf1:c1=pvalue1
cbt -instance instance-1 -project project-1  set table-1 pey2 cf1:c1=pvalue2
cbt -instance instance-1 -project project-1  set table-1 pey3 cf1:c1=pvalue3
cbt -instance instance-1 -project project-1  set table-1 pey4 cf1:c1=pvalue4
cbt -instance instance-1 -project project-1  set table-1 pey5 cf1:c1=pvalue5
cbt -instance instance-1 -project project-1  set table-1 pey6 cf1:c1=pvalue6

# Set another version into key1
cbt -instance instance-1 -project project-1  set table-1 key1 cf1:c1=value1.v1
cbt -instance instance-1 -project project-1  set table-1 key1 cf1:c2=value1.c2

# Set another qualifier on key2
cbt -instance instance-1 -project project-1  set table-1 key2 cf1:c2=value2.c2

# Set another family on key3
cbt -instance instance-1 -project project-1  set table-1 key3 cf2:c1=value3.cf2


cbt -instance instance-1 -project project-1  read table-1