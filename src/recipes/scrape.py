from recipe_scrapers import scrape_me
import re


def scrape_recipe(url):
    scraper = scrape_me(url, wild_mode=True)
    s = ""
    snake_cased_title = scraper.title().replace(
        ' ', '-').replace('(', '').replace(')', '').replace('&', 'and').lower()
    snake_cased_title = re.sub('[^a-zA-Z]', '', snake_cased_title)
    snake_cased_title = snake_cased_title + '.md'

    if scraper.image() and scraper.title():
        s += '---\n'
        s += f'title: {scraper.title()}\n'
        s += '---\n'
        s += '\n'
        s += f"![{scraper.title()}]({scraper.image()})\n\n"
    if scraper.title():
        s += f"# {scraper.title()}\n\n"
    if scraper.url:
        s += f"- From: [link]({scraper.url})\n\n"
    if scraper.total_time():
        s += f"- Cooking Time: {scraper.total_time()} minutes\n\n"
    if scraper.ingredients():
        s += "## Ingredients:\n\n"
        for i in scraper.ingredients():
            s += f"- {i}\n"
        s += '\n'
    if scraper.instructions():
        s += "## Instructions:\n\n"
        s += scraper.instructions().replace('\n', '\n\n') + '\n'

    with open(f"recipes/{snake_cased_title}", 'w+') as f:
        f.write(s)

    return (snake_cased_title, scraper.title().replace('(', '').replace(')', '').replace('&', 'and'))


recipes = []

with open('recipes_to_scrape.txt') as f:
    for line in f:
        if line:
            recipes.append(scrape_recipe(line.strip()))

with open('./recipes/_recipes.md', 'a') as f:
    for (file_name, title) in recipes:
        f.write(f"- [{title.title()}](./{file_name[:-3]})\n")
