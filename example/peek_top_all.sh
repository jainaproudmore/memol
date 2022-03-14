memol clear
memol push "I do something Task1"
memol push "I do something Task2"
memol push "I do something Task3"

echo ""
memol all
echo ""
memol all -n 1
echo ""
memol all -r -n 2
echo ""
memol peek # memol top
echo ""
memol peek -r # memol top -r
