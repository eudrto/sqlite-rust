#!/bin/sh

FLDR=dbs
DB=mountains

rm $FLDR/$DB.db
sqlite3 $FLDR/$DB.db < $FLDR/$DB.sql
