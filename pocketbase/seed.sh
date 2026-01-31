#!/bin/bash
set -e

BASE_URL="http://127.0.0.1:8090/api"

echo "Seeding zones..."
curl -s -X POST "$BASE_URL/collections/zones/records" \
  -H "Content-Type: application/json" \
  -d '{"name":"Sonntag - Frei","emoji":"🛋️","weekday":0,"color":"#9333EA"}' >/dev/null

curl -s -X POST "$BASE_URL/collections/zones/records" \
  -H "Content-Type: application/json" \
  -d '{"name":"Montag (HO) - EG","emoji":"🏠","weekday":1,"color":"#3B82F6"}' >/dev/null

curl -s -X POST "$BASE_URL/collections/zones/records" \
  -H "Content-Type: application/json" \
  -d '{"name":"Dienstag (HO) - KG","emoji":"🧺","weekday":2,"color":"#10B981"}' >/dev/null

curl -s -X POST "$BASE_URL/collections/zones/records" \
  -H "Content-Type: application/json" \
  -d '{"name":"Mittwoch (HO) - OG","emoji":"🛁","weekday":3,"color":"#F59E0B"}' >/dev/null

curl -s -X POST "$BASE_URL/collections/zones/records" \
  -H "Content-Type: application/json" \
  -d '{"name":"Donnerstag (Büro) - Leicht","emoji":"📋","weekday":4,"color":"#EF4444"}' >/dev/null

curl -s -X POST "$BASE_URL/collections/zones/records" \
  -H "Content-Type: application/json" \
  -d '{"name":"Freitag (Büro) - Reset","emoji":"🔄","weekday":5,"color":"#8B5CF6"}' >/dev/null

curl -s -X POST "$BASE_URL/collections/zones/records" \
  -H "Content-Type: application/json" \
  -d '{"name":"Samstag - Frei","emoji":"🎉","weekday":6,"color":"#EC4899"}' >/dev/null

echo "✓ Zones seeded"

MONDAY_ID=$(curl -s "$BASE_URL/collections/zones/records?filter=weekday=1" | jq -r '.items[0].id')
TUESDAY_ID=$(curl -s "$BASE_URL/collections/zones/records?filter=weekday=2" | jq -r '.items[0].id')
WEDNESDAY_ID=$(curl -s "$BASE_URL/collections/zones/records?filter=weekday=3" | jq -r '.items[0].id')
THURSDAY_ID=$(curl -s "$BASE_URL/collections/zones/records?filter=weekday=4" | jq -r '.items[0].id')
FRIDAY_ID=$(curl -s "$BASE_URL/collections/zones/records?filter=weekday=5" | jq -r '.items[0].id')

echo "Seeding tasks..."

curl -s -X POST "$BASE_URL/collections/tasks/records" -H "Content-Type: application/json" -d '{"name":"Spülmaschine anstellen","emoji":"🍽️","is_daily":true,"sort_order":1,"completed":false}' >/dev/null
curl -s -X POST "$BASE_URL/collections/tasks/records" -H "Content-Type: application/json" -d '{"name":"Spülmaschine ausräumen","emoji":"🍴","is_daily":true,"sort_order":2,"completed":false}' >/dev/null
curl -s -X POST "$BASE_URL/collections/tasks/records" -H "Content-Type: application/json" -d '{"name":"Küche grob aufräumen","emoji":"🧹","is_daily":true,"sort_order":3,"completed":false}' >/dev/null
curl -s -X POST "$BASE_URL/collections/tasks/records" -H "Content-Type: application/json" -d '{"name":"1 Wäschegang (waschen + aufhängen)","emoji":"👕","is_daily":true,"sort_order":4,"completed":false}' >/dev/null
curl -s -X POST "$BASE_URL/collections/tasks/records" -H "Content-Type: application/json" -d '{"name":"5 Min Aufräumen (gemeinsam)","emoji":"⏱️","is_daily":true,"sort_order":5,"completed":false}' >/dev/null
curl -s -X POST "$BASE_URL/collections/tasks/records" -H "Content-Type: application/json" -d '{"name":"Alle Oberflächen frei","emoji":"✨","is_daily":true,"sort_order":6,"completed":false}' >/dev/null

curl -s -X POST "$BASE_URL/collections/tasks/records" -H "Content-Type: application/json" -d "{\"name\":\"Küche: Herd, Spüle, Arbeitsflächen\",\"emoji\":\"🏠\",\"zone\":\"$MONDAY_ID\",\"is_daily\":false,\"sort_order\":10,\"completed\":false}" >/dev/null
curl -s -X POST "$BASE_URL/collections/tasks/records" -H "Content-Type: application/json" -d "{\"name\":\"Esstisch abwischen\",\"emoji\":\"🍽️\",\"zone\":\"$MONDAY_ID\",\"is_daily\":false,\"sort_order\":11,\"completed\":false}" >/dev/null
curl -s -X POST "$BASE_URL/collections/tasks/records" -H "Content-Type: application/json" -d "{\"name\":\"WC putzen\",\"emoji\":\"🚽\",\"zone\":\"$MONDAY_ID\",\"is_daily\":false,\"sort_order\":12,\"completed\":false}" >/dev/null
curl -s -X POST "$BASE_URL/collections/tasks/records" -H "Content-Type: application/json" -d "{\"name\":\"EG durchsaugen & wischen\",\"emoji\":\"🧽\",\"zone\":\"$MONDAY_ID\",\"is_daily\":false,\"sort_order\":13,\"completed\":false}" >/dev/null

curl -s -X POST "$BASE_URL/collections/tasks/records" -H "Content-Type: application/json" -d "{\"name\":\"Waschmaschine reinigen\",\"emoji\":\"🧺\",\"zone\":\"$TUESDAY_ID\",\"is_daily\":false,\"sort_order\":20,\"completed\":false}" >/dev/null
curl -s -X POST "$BASE_URL/collections/tasks/records" -H "Content-Type: application/json" -d "{\"name\":\"Müll rausbringen (alle Eimer)\",\"emoji\":\"🗑️\",\"zone\":\"$TUESDAY_ID\",\"is_daily\":false,\"sort_order\":21,\"completed\":false}" >/dev/null
curl -s -X POST "$BASE_URL/collections/tasks/records" -H "Content-Type: application/json" -d "{\"name\":\"KG aufräumen & durchsaugen\",\"emoji\":\"🏚️\",\"zone\":\"$TUESDAY_ID\",\"is_daily\":false,\"sort_order\":22,\"completed\":false}" >/dev/null

curl -s -X POST "$BASE_URL/collections/tasks/records" -H "Content-Type: application/json" -d "{\"name\":\"Badezimmer putzen (Dusche, Waschbecken)\",\"emoji\":\"🛁\",\"zone\":\"$WEDNESDAY_ID\",\"is_daily\":false,\"sort_order\":30,\"completed\":false}" >/dev/null
curl -s -X POST "$BASE_URL/collections/tasks/records" -H "Content-Type: application/json" -d "{\"name\":\"Betten neu beziehen\",\"emoji\":\"🛏️\",\"zone\":\"$WEDNESDAY_ID\",\"is_daily\":false,\"sort_order\":31,\"completed\":false}" >/dev/null
curl -s -X POST "$BASE_URL/collections/tasks/records" -H "Content-Type: application/json" -d "{\"name\":\"Wäsche zusammenlegen\",\"emoji\":\"👔\",\"zone\":\"$WEDNESDAY_ID\",\"is_daily\":false,\"sort_order\":32,\"completed\":false}" >/dev/null
curl -s -X POST "$BASE_URL/collections/tasks/records" -H "Content-Type: application/json" -d "{\"name\":\"OG durchsaugen\",\"emoji\":\"🧹\",\"zone\":\"$WEDNESDAY_ID\",\"is_daily\":false,\"sort_order\":33,\"completed\":false}" >/dev/null

curl -s -X POST "$BASE_URL/collections/tasks/records" -H "Content-Type: application/json" -d "{\"name\":\"Staub wischen (sichtbare Flächen)\",\"emoji\":\"🪶\",\"zone\":\"$THURSDAY_ID\",\"is_daily\":false,\"sort_order\":40,\"completed\":false}" >/dev/null
curl -s -X POST "$BASE_URL/collections/tasks/records" -H "Content-Type: application/json" -d "{\"name\":\"Papierkram sortieren\",\"emoji\":\"📄\",\"zone\":\"$THURSDAY_ID\",\"is_daily\":false,\"sort_order\":41,\"completed\":false}" >/dev/null

curl -s -X POST "$BASE_URL/collections/tasks/records" -H "Content-Type: application/json" -d "{\"name\":\"Restmüll + Biomüll raus\",\"emoji\":\"♻️\",\"zone\":\"$FRIDAY_ID\",\"is_daily\":false,\"sort_order\":50,\"completed\":false}" >/dev/null
curl -s -X POST "$BASE_URL/collections/tasks/records" -H "Content-Type: application/json" -d "{\"name\":\"Wäsche von Woche zusammenlegen\",\"emoji\":\"👕\",\"zone\":\"$FRIDAY_ID\",\"is_daily\":false,\"sort_order\":51,\"completed\":false}" >/dev/null
curl -s -X POST "$BASE_URL/collections/tasks/records" -H "Content-Type: application/json" -d "{\"name\":\"Gäste-WC Check\",\"emoji\":\"🚿\",\"zone\":\"$FRIDAY_ID\",\"is_daily\":false,\"sort_order\":52,\"completed\":false}" >/dev/null

curl -s -X POST "$BASE_URL/collections/tasks/records" -H "Content-Type: application/json" -d '{"name":"Bad gründliche Reinigung (Fugen, Armaturen)","emoji":"🧼","is_daily":false,"sort_order":100,"completed":false}' >/dev/null
curl -s -X POST "$BASE_URL/collections/tasks/records" -H "Content-Type: application/json" -d '{"name":"Kühlschrank ausputzen","emoji":"❄️","is_daily":false,"sort_order":101,"completed":false}' >/dev/null
curl -s -X POST "$BASE_URL/collections/tasks/records" -H "Content-Type: application/json" -d '{"name":"Fenster putzen (alle)","emoji":"🪟","is_daily":false,"sort_order":102,"completed":false}' >/dev/null
curl -s -X POST "$BASE_URL/collections/tasks/records" -H "Content-Type: application/json" -d '{"name":"Schrank aussortieren (Kleidung, Spielzeug)","emoji":"📦","is_daily":false,"sort_order":103,"completed":false}' >/dev/null

echo "✓ Tasks seeded"

ZONES_COUNT=$(curl -s "$BASE_URL/collections/zones/records" | jq '.totalItems')
TASKS_COUNT=$(curl -s "$BASE_URL/collections/tasks/records" | jq '.totalItems')

echo ""
echo "Summary:"
echo "- Zones: $ZONES_COUNT"
echo "- Tasks: $TASKS_COUNT"
