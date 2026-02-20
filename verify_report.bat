curl -v -X POST http://localhost:3001/api/reports/generate -H "Content-Type: application/json" --cookie "axur_session=dummy; axur_user=test@example.com" -d @request.json -o mock_plugin_report.html
