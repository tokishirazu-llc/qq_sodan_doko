#!/usr/bin/env python3

# pip install line-bot-sdk
# pip install pyyaml
import sys
import requests
import yaml
from linebot import (
    LineBotApi
)
from linebot.models import (
    RichMenu, RichMenuArea, RichMenuSize, RichMenuBounds,
    PostbackAction, URIAction,
)


def main():
    if len(sys.argv) <= 1:
        print('richmenu/richmenu.py dev')
        return 

    env = sys.argv[1]

    with open('conf/'+env+'.yml') as file:
        yml = yaml.safe_load(file)

    access_token = yml['LINE_BOT_ACCESS_TOKEN']

    line_bot_api = LineBotApi(access_token)

    line_bot_api.cancel_default_rich_menu()

    delete_richmenu(line_bot_api)
    id = create_user_richmenu(line_bot_api)
    upload_user_richmenu_image(line_bot_api, id)

    line_bot_api.set_default_rich_menu(id)
    
    return

def delete_richmenu(line_bot_api):
    print("delete user richmenu")
    menu_list = line_bot_api.get_rich_menu_list()

    for richmenu in menu_list:
        print("delete user richmenu "+richmenu.rich_menu_id)
        line_bot_api.delete_rich_menu(richmenu.rich_menu_id)
        
    
                    
def create_user_richmenu(line_bot_api):
    print("create user richmenu")
    user_menu = RichMenu(
        size=RichMenuSize(width=2500, height=843),
        selected=True,
        name="qq_sodan_doko user menu",
        chat_bar_text="メニューはこちら",
        areas=[
            RichMenuArea(
                bounds=RichMenuBounds(
                    x=0,
                    y=0,
                    width=833,
                    height=843,
                ),
                action=URIAction(
                    label='location',
                    uri="https://line.me/R/nv/location/",
                ),
            ),
            RichMenuArea(
                bounds=RichMenuBounds(
                    x=833,
                    y=0,
                    width=833,
                    height=843,
                ),
                action=URIAction(
                    label='qsuke',
                    uri="http://www.fdma.go.jp/neuter/topics/filedList9_6/kyukyu_app/kyukyu_app_web/index.html",
                ),
            ),
            RichMenuArea(
                bounds=RichMenuBounds(
                    x=1666,
                    y=0,
                    width=833,
                    height=843,
                ),
                action=URIAction(
                    label='hp',
                    uri="https://github.com/alivelime/qq_sodan_doko",
                ),
            ),
        ]
    )
    id = line_bot_api.create_rich_menu(rich_menu=user_menu)
    print("user richmenu id = "+id)
    return id

def upload_user_richmenu_image(line_bot_api, id):
    print("update user richmenu "+ id)
    with open('richmenu/richmenu.png', 'rb') as f:
        line_bot_api.set_rich_menu_image(id, 'image/png', f)

main()
