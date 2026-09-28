import requests
import json
import sys

BASE_URL = "http://127.0.0.1:3000/api/v1"

def test_orgs():
    print("Verifying Org Endpoints...")

    try:
        # 1. Create Org
        print("\nTesting POST /orgs...")
        org_payload = {"username": "test-org", "description": "Test Organization"}
        r = requests.post(f"{BASE_URL}/orgs", json=org_payload)
        r.raise_for_status()
        org = r.json()
        print(f"Created Org: {org['id']} - {org['username']}")

        # 2. Get Org
        print(f"\nTesting GET /orgs/{org['username']}...")
        r = requests.get(f"{BASE_URL}/orgs/{org['username']}")
        r.raise_for_status()
        fetched_org = r.json()
        if fetched_org['username'] == "test-org":
            print("SUCCESS: Fetched org")
        else:
            print("FAILURE: Fetched wrong org")
            sys.exit(1)

        # 3. Create Team
        print(f"\nTesting POST /orgs/{org['username']}/teams...")
        team_payload = {"name": "Developers", "description": "Dev Team", "permission": "write"}
        r = requests.post(f"{BASE_URL}/orgs/{org['username']}/teams", json=team_payload)
        r.raise_for_status()
        team = r.json()
        print(f"Created Team: {team['id']} - {team['name']}")

        # 4. List Teams
        print(f"\nTesting GET /orgs/{org['username']}/teams...")
        r = requests.get(f"{BASE_URL}/orgs/{org['username']}/teams")
        r.raise_for_status()
        teams = r.json()
        print(f"Teams count: {len(teams)}")

        found = any(t['name'] == "Developers" for t in teams)
        if found:
            print("SUCCESS: Team found in list")
        else:
            print("FAILURE: Team not found")
            sys.exit(1)

    except Exception as e:
        print(f"Error testing orgs: {e}")
        sys.exit(1)

if __name__ == "__main__":
    test_orgs()
