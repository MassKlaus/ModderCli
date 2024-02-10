@echo off

REM Clean the project

del  /s /q .info
del  /s /q .ignore
RMDIR /s /q branches
RMDIR /s /q publish
RMDIR /s /q assets
RMDIR /s /q testground

