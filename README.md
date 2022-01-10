# Concurrent Intcode Webserver

## Beschreibung

Einen Webserver mit Rust implementieren, welcher in der Lage ist Intcode-Anfragen concurrent zu bearbeiten. 

Bei einem Intcode handelt es sich um eine Liste mit Integerwerten. Diese werden in 4er Blöcken verarbeitet. Dabei stellt die erste Zahl den Opcode dar, welcher für die nachfolgende Operation entscheidend ist.

Der Opcode 1 steht hierbei z.B. für Addition, 2 für Multiplikation und 99 für Beenden des Programms.

Die nächsten zwei Zahlen, sind die zu lesenden Elemente für die im Opcode definierte Operation. Die 4. Zahl bestimmt wo das Ergebnis gespeichert wird.

Nach Abarbeitung werden die nächsten 4 Elemente der Liste verarbeitet, solange bis der Opcode 99 das Programm erfolgreich beendet, oder ein unbekannter Opcode zum Abbruch führt.

Inspieriert durch  https://adventofcode.com/2019 ( Aufgaben 2,5 und 7) und kann Beliebig komplex werden( z.B. durch Hinzufügen des Parametermodes in Aufgabe 5)

## Beispiel

Für die Beispielliste 1,4,5,9,10,..... 

wäre die Abarbeitung also die Addition(Opcode 1)  von 9 und 10 ( Den Werten an Postion 4 und 5) und das schreiben des Ergebnisses an Position 9.


