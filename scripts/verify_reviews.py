import requests
import json
import sys

BASE_URL = "http://127.0.0.1:3000/api/v1"

def test_reviews():
    print("Verifying Review Endpoints...")
    repo_owner = "admin"
    repo_name = "codeza"

    try:
        # 1. Create PR
        print("\nTesting POST /repos/:owner/:repo/pulls...")
        pr_payload = {"title": "Review PR", "body": "Please review", "head": "feature", "base": "main"}
        r = requests.post(f"{BASE_URL}/repos/{repo_owner}/{repo_name}/pulls", json=pr_payload)
        r.raise_for_status()
        pr = r.json()
        print(f"Created PR: {pr['number']} - {pr['title']}")
        pr_number = pr['number']

        # 2. Post Review
        print(f"\nTesting POST /repos/:owner/:repo/pulls/{pr_number}/reviews...")
        review_payload = {"body": "LGTM", "event": "APPROVE"}
        r = requests.post(f"{BASE_URL}/repos/{repo_owner}/{repo_name}/pulls/{pr_number}/reviews", json=review_payload)
        r.raise_for_status()
        review = r.json()
        print(f"Created Review: {review['id']} - {review['state']}")

        # 3. List Reviews
        print(f"\nTesting GET /repos/:owner/:repo/pulls/{pr_number}/reviews...")
        r = requests.get(f"{BASE_URL}/repos/{repo_owner}/{repo_name}/pulls/{pr_number}/reviews")
        r.raise_for_status()
        reviews = r.json()
        print(f"Reviews count: {len(reviews)}")

        found = any(rv['body'] == "LGTM" and rv['state'] == "APPROVED" for rv in reviews)
        if found:
            print("SUCCESS: Review found in list")
        else:
            print("FAILURE: Review not found")
            sys.exit(1)

    except Exception as e:
        print(f"Error testing reviews: {e}")
        sys.exit(1)

if __name__ == "__main__":
    test_reviews()
