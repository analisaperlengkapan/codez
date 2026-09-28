import requests
import json
import sys

BASE_URL = "http://127.0.0.1:3000/api/v1"

def test_backend():
    print("Verifying Backend Endpoints...")

    # 1. List User Issues
    print("\nTesting GET /user/issues...")
    try:
        # We need to create an issue assigned to 'admin' (id=1) first because mock might be empty or specific
        # Create issue
        repo_owner = "admin"
        repo_name = "codeza"
        issue_payload = {"title": "Verification Issue", "body": "To test list"}
        r = requests.post(f"{BASE_URL}/repos/{repo_owner}/{repo_name}/issues", json=issue_payload)
        r.raise_for_status()
        issue = r.json()
        print(f"Created Issue: {issue['id']}")

        # Assign to admin
        user_payload = {"id": 1, "username": "admin", "email": None}
        r = requests.post(f"{BASE_URL}/repos/{repo_owner}/{repo_name}/issues/{issue['id']}/assignees", json=user_payload)
        r.raise_for_status()
        print("Assigned to admin")

        # Now list user issues
        r = requests.get(f"{BASE_URL}/user/issues?state=open")
        r.raise_for_status()
        issues = r.json()
        print(f"User Issues Count: {len(issues)}")

        found = any(i['title'] == "Verification Issue" for i in issues)
        if found:
            print("SUCCESS: Found assigned issue in list")
        else:
            print("FAILURE: Did not find assigned issue")
            # print(json.dumps(issues, indent=2))
            sys.exit(1)

    except Exception as e:
        print(f"Error testing user issues: {e}")
        sys.exit(1)

    # 2. List User Pull Requests
    print("\nTesting GET /user/pulls...")
    try:
        # Create PR (admin is default creator)
        pr_payload = {"title": "Verification PR", "body": "Body", "head": "feature", "base": "main"}
        r = requests.post(f"{BASE_URL}/repos/{repo_owner}/{repo_name}/pulls", json=pr_payload)
        r.raise_for_status()
        pr = r.json()
        print(f"Created PR: {pr['id']}")

        # List user pulls
        r = requests.get(f"{BASE_URL}/user/pulls?state=open")
        r.raise_for_status()
        pulls = r.json()
        print(f"User Pulls Count: {len(pulls)}")

        found = any(p['title'] == "Verification PR" for p in pulls)
        if found:
            print("SUCCESS: Found created PR in list")
        else:
            print("FAILURE: Did not find created PR")
            sys.exit(1)

    except Exception as e:
        print(f"Error testing user pulls: {e}")
        sys.exit(1)

if __name__ == "__main__":
    test_backend()
