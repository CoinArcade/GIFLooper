#!/bin/bash

PROJ_NAME="bugout.color-chooser"

rm -f ./build/libs/$PROJ_NAME*.jar
gradle build
cp ./build/libs/$PROJ_NAME*.jar $PROJ_NAME.jar
