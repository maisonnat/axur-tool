---
description: Validacion previa a un commit
---

name: "Secure Commit"
description: "Realiza un escaneo de seguridad antes de hacer commit"
inputs:
  message:
    description: "Mensaje del commit"
steps:
  # Paso 1: Verificar el status
  - action: shell.execute
    command: "git status"
  
  # Paso 2: La IA revisa el diff buscando patrones peligrosos (instrucción implícita al agente)
  - action: agent.review
    prompt: "Revisa la salida de 'git diff --cached'. Busca EXHAUSTIVAMENTE cualquier credencial, token o path absoluto. Si encuentras algo sospechoso, DETÉN el proceso y avísame."

  # Paso 3: Si la IA aprueba (no hay error), ejecuta el commit
  - action: shell.execute
    command: "git commit -m '{{message}}'"