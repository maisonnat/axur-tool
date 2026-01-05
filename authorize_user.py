#!/usr/bin/env python3
"""
Script to perform initial OAuth2 flow for a Desktop App.
It will generate a token.json file containing the Refresh Token.
"""

import os.path
import json

from google.auth.transport.requests import Request
from google.oauth2.credentials import Credentials
from google_auth_oauthlib.flow import InstalledAppFlow

# Valid scopes for Drive and Slides
SCOPES = [
    'https://www.googleapis.com/auth/drive.file',
    'https://www.googleapis.com/auth/presentations.readonly'
]

CLIENT_SECRET_FILE = 'config/client_secret.json'
TOKEN_FILE = 'config/token.json'

def main():
    creds = None
    # The file token.json stores the user's access and refresh tokens, and is
    # created automatically when the authorization flow completes for the first
    # time.
    if os.path.exists(TOKEN_FILE):
        creds = Credentials.from_authorized_user_file(TOKEN_FILE, SCOPES)
    
    # If there are no (valid) credentials available, let the user log in.
    if not creds or not creds.valid:
        if creds and creds.expired and creds.refresh_token:
            creds.refresh(Request())
        else:
            flow = InstalledAppFlow.from_client_secrets_file(
                CLIENT_SECRET_FILE, SCOPES)
            creds = flow.run_local_server(port=0)
        
        # Save the credentials for the next run
        with open(TOKEN_FILE, 'w') as token:
            token.write(creds.to_json())
            
    print(f"\n✅ Authentication successful!")
    print(f"Token saved to: {TOKEN_FILE}")
    print(f"Refresh Token: {creds.refresh_token}")

if __name__ == '__main__':
    main()
